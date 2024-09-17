{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        # isn't cooperating 3:<
        # uniffi-bindgen-go = pkgs.rustPlatform.buildRustPackage rec {
        #   pname = "uniffi-bindgen-go";
        #   version = "v0.2.1+v0.25.0";
        #   src = pkgs.fetchFromGitHub {
        #     owner = "NordSecurity";
        #     repo = pname;
        #     rev = version;
        #     deepClone = true;
        #     fetchSubmodules = true;
        #     sha256 = "sha256-f3KRtJ0Y3HYJm+YoytE3sj+NFSVQHkqkcTgwftX/nN8=";
        #   };
        #   cargoHash = "sha256-RByHAg5WjUflAoCIZ3m1eB3rYsCVwHEZ8cIxaZqf40c=";
        # };
      in
      {
        devShells.default = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            # Go
            go
            # Rust
            cargo
            rustc
            rustfmt
            clippy
            # To find libraries
            pkg-config
            # Libraries
            protobuf
            zlib
            # Build tools
            # uniffi-bindgen-go
          ];
          # Environment variable with the path to all our libraries
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
