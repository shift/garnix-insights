{ 
  description = "A simple Rust project"; 
 
  inputs = { 
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; 
    flake-utils.url = "github:numtide/flake-utils"; 
    crane.url = "github:ipetkov/crane"; 
    crane.inputs.nixpkgs.follows = "nixpkgs"; 
 
    rust-overlay = { 
      url = "github:oxalica/rust-overlay"; 
      inputs.nixpkgs.follows = "nixpkgs"; 
    }; 
  }; 
 
  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay }: 
    flake-utils.lib.eachDefaultSystem (system: 
      let 
        pkgs = import nixpkgs { 
          inherit system; 
          overlays = [ (import rust-overlay) ]; 
        }; 
        inherit (pkgs) lib; 
 
        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default); 
        src = craneLib.cleanCargoSource ./.; 
 
        crateExpression = { openssl, pkg-config, stdenv }: 
          craneLib.buildPackage { 
            pname = "garnix-fetcher"; 
            version = "0.1.0"; 
            inherit src; 
 
            cargoLock = ./Cargo.lock; 
            doCheck = false; 
 
            nativeBuildInputs = [ 
              pkg-config 
            ]; 
 
            buildInputs = [ 
              openssl 
            ]; 
 
            installPhase = '' 
              mkdir -p $out/bin 
              cp target/release/garnix-fetcher $out/bin/ 
            ''; 
          }; 
 
        garnixFetcher = pkgs.callPackage crateExpression { }; # The main executable 
      in 
      { 
        devShells.default = pkgs.mkShell { 
          packages = with pkgs; [ 
            rustc 
            cargo 
            gcc 
          ]; 
        }; 
 
        packages.default = garnixFetcher; # The CLI executable 
 
        apps.default = flake-utils.lib.mkApp { 
          drv = garnixFetcher; # Default app is the CLI 
        }; 
 
        apps.server = flake-utils.lib.mkApp { 
          drv = pkgs.writeScriptBin "garnix-api-server" '' 
            #!${pkgs.bash}/bin/bash 
            exec ${garnixFetcher}/bin/garnix-fetcher --server "$@" 
          ''; 
        }; 
      } 
    ); 
}