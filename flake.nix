{ 
  description = "A simple Rust project"; 
 
  inputs = { 
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; 
    flake-utils.url = "github:numtide/flake-utils"; 
    crane.url = "github:ipetkov/crane"; # Add crane input 
    crane.inputs.nixpkgs.follows = "nixpkgs"; # Ensure crane uses the same nixpkgs 
  }; 
 
  outputs = { self, nixpkgs, flake-utils, crane }: # Add crane to outputs 
    flake-utils.lib.eachDefaultSystem (system: 
      let 
        pkgs = nixpkgs.legacyPackages.${system}; 
        craneLib = crane.lib.${system}; # Get crane library for the system 
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
 
        packages.default = craneLib.buildRustPackage { # Use craneLib.buildRustPackage 
          pname = "garnix-fetcher"; 
          version = "0.1.0"; 
          src = ./.; # Source is the current directory 
 
          cargoLock = { 
            lockFile = ./Cargo.lock; # Path to Cargo.lock 
          }; 
 
          # crane handles release builds automatically 
          # cargoBuildFlags = [ ]; 
 
          # Build inputs for openssl-sys and pkg-config 
          buildInputs = with pkgs; [ 
            pkg-config 
            openssl 
            pkgs.lib.getLib pkgs.openssl # Explicitly get the library 
          ]; 
 
          # Remove explicit OpenSSL environment variables 
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig"; 
          # OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib"; 
          # OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include"; 
 
          # Install the binary 
          installPhase = '' 
            mkdir -p $out/bin 
            cp target/release/garnix-fetcher $out/bin/ 
          ''; 
        }; 
      } 
    ); 
}