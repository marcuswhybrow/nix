{
  description = "Rust project packaged using Naersk and rust-overlay";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
  };
  outputs = inputs: let 
    pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
    toolchain = inputs.fenix.packages.x86_64-linux.combine [
      inputs.fenix.packages.x86_64-linux.latest.toolchain
      # inputs.fenix.packages.x86_64-linux.wasm32-unknown-unknown.latest.rust-std
    ];
    naersk = pkgs.callPackage inputs.naersk {
      cargo = toolchain;
      rustc = toolchain;
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
        toolchain
        pkgs.rust-analyzer
        # pkgs.bacon 
      ];
    };
  };
}
