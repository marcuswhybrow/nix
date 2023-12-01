{
  description = "Marcus's Nix utils";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = inputs: let 
    pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;

    tmplEntries = builtins.readDir ./templates;
    tmplDirs = pkgs.lib.filterAttrs (n: v: v == "directory") tmplEntries;
    tmplFlake = name: import (./templates + "/${name}/flake.nix");
    dirToTmplOut = n: v: { 
      path = ./templates + "/${n}"; 
      description = (tmplFlake n).description;
    };
  in {
    templates = pkgs.lib.mapAttrs dirToTmplOut tmplDirs;
  };
}
