{ 
  description = "A simple Rust project"; 
 
  inputs = { 
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; 
    flake-utils.url = "github:numtide/flake-utils"; 
    crane.url = "github:ipetkov/crane"; # Add crane input 
    crane.inputs.nixpkgs.follows = "nixpkgs"; # Ensure crane uses the same nixpkgs 
 
    rust-overlay = { 
      url = "github:oxalica/rust-overlay"; 
      inputs.nixpkgs.follows = "nixpkgs"; 
    }; 
  }; 
 
  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay }: # Add rust-overlay to outputs 
    flake-utils.lib.eachDefaultSystem (system: 
      let 
        pkgs = import nixpkgs { 
          inherit system; 
          overlays = [ (import rust-overlay) ]; 
        }; 
        inherit (pkgs) lib; 
 
        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default); 
        src = craneLib.cleanCargoSource ./.; 
      in 
      { 
        devShells.default = pkgs.mkShell { 
          packages = with pkgs; [ 
            rustc 
            cargo 
            pkg-config 
            openssl 
            gcc # Add gcc 
          ]; 
        }; 
 
        packages.default = craneLib.buildPackage { # Use craneLib.buildPackage 
          pname = "garnix-fetcher"; 
          version = "0.1.0"; 
          inherit src; # Use cleaned source 
 
          cargoLock = { 
            lockFile = ./Cargo.lock; # Path to Cargo.lock 
          }; 
 
          doCheck = false; # Disable tests during build 
 
          # Install the binary 
          installPhase = '' 
            mkdir -p $out/bin 
            cp target/release/garnix-fetcher $out/bin/ 
          ''; 
        }; 
      } 
    ); 
}