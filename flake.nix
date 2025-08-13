{ 
  description = "A simple Rust project"; 
 
  inputs = { 
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; 
    flake-utils.url = "github:numtide/flake-utils"; 
  }; 
 
  outputs = { self, nixpkgs, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system: 
      let 
        pkgs = nixpkgs.legacyPackages.${system}; 
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
 
        packages.default = pkgs.rustPlatform.buildRustPackage { 
          pname = "garnix-fetcher"; 
          version = "0.1.0"; 
          src = ./.; # Source is the current directory 
 
          cargoLock = { 
            lockFile = ./Cargo.lock; # Path to Cargo.lock 
          }; 
 
          # Build in release mode 
          cargoBuildFlags = [ ]; 
 
          # Build inputs for openssl-sys and pkg-config 
          buildInputs = with pkgs; [ 
            pkg-config 
            openssl 
          ]; 
 
          # Explicitly set OPENSSL_DIR for openssl-sys 
          OPENSSL_DIR = pkgs.openssl.dev; 
 
          # Explicitly link OpenSSL libraries 
          rustcBuildFlags = [ 
            "-L${pkgs.openssl.lib}/lib" 
            "-lssl" 
            "-lcrypto" 
          ]; 
 
          # Install the binary 
          installPhase = '' 
            mkdir -p $out/bin 
            cp target/release/garnix-fetcher $out/bin/ 
          ''; 
        }; 
      } 
    ); 
}