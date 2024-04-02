{
  description = "Desktop entry example launches firefox in private mode";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }: {

    packages.x86_64-linux.firefox-private = let 
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in pkgs.runCommand "firefox-private" {
      nativeBuildInputs = [ pkgs.makeWrapper ];
    } ''
      mkdir -p $out/bin
      makeWrapper \
        ${pkgs.firefox}/bin/firefox \
        $out/bin/firefox-private \
        --add-flags "--private-window"

      mkdir -p $out/share/applications
      cat > $out/share/applications/firefox-private.desktop << EOF
      [Desktop Entry]
      Version=1.0
      Name=Firefox Private
      GenericName=Launch Firefox in private mode
      Terminal=false
      Type=Application
      Exec=$out/bin/firefox-private
      EOF

    '';

    packages.x86_64-linux.default = self.packages.x86_64-linux.firefox-private;
  };
}
