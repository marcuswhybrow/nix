{
  description = "Bash Script";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs: let 
    pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
  in {
    packages.x86_64-linux.script = pkgs.writeShellScriptBin "script" ''
      echo "Hello, world!"
    '';

    packages.x86_64-linux.default = inputs.self.packages.x86_64-linux.script;

    devShells.x86_64-linux.default = pkgs.mkShell {
      packages = [];
    };
  };
}
