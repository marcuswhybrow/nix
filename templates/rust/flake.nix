{
  description = "Rust project packaged using Naersk and rust-overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = inputs: let 
    pkgs = import inputs.nixpkgs { 
      system = "x86_64-linux"; 
      overlays = [ 
        inputs.rust-overlay.overlays.default 
      ];
    };


    # https://github.com/oxalica/rust-overlay#cheat-sheet-common-usage-of-rust-bin
    rust-bin = pkgs.rust-bin.stable.latest.default; 

    # https://github.com/nix-community/naersk
    naersk = pkgs.callPackage inputs.naersk {
      cargo = rust-bin;
      rustc = rust-bin;
    };

    cargo = (pkgs.lib.importTOML ./Cargo.toml).package;
  in {
    packages.x86_64-linux.rust = naersk.buildPackage {
      name = cargo.name;
      version = cargo.version;
      src = pkgs.lib.cleanSource ./.;
    };
    packages.x86_64-linux.default = inputs.self.packages.x86_64-linux.rust;

    devShells.x86_64-linux.default = pkgs.mkShell {
      nativeBuildInputs = [ 
        rust-bin
      ];
    };
  };
}
