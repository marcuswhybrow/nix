{
  description = "Rust eframe/egui GUI template with Windows cross compilation";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
  };
  outputs = inputs: let 
    pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
    toolchain = inputs.fenix.packages.x86_64-linux.combine [
      inputs.fenix.packages.x86_64-linux.latest.toolchain
      inputs.fenix.packages.x86_64-linux.targets.x86_64-pc-windows-gnu.latest.rust-std
    ];
    naersk = pkgs.callPackage inputs.naersk {
      cargo = toolchain;
      rustc = toolchain;
    };
    cargo = (pkgs.lib.importTOML ./Cargo.toml).package;
    ldLibPath = pkgs.lib.makeLibraryPath [
      pkgs.libx11
      pkgs.libxcursor
      pkgs.libxrandr
      pkgs.libxi
      pkgs.libxext
      pkgs.libxkbcommon
      pkgs.libGL
      pkgs.mesa
      pkgs.vulkan-loader
    ];
    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
    pkgsCrossWindows = import inputs.nixpkgs {
      system = "x86_64-linux";
      crossSystem.config = "x86_64-w64-mingw32";
      crossSystem.libc = "msvcrt";
    };
    mingw = pkgs.pkgsCross.mingwW64;
  in {
    packages.x86_64-linux.eframe-app = naersk.buildPackage {
      name = cargo.name;
      version = cargo.version;
      src = pkgs.lib.cleanSource ./.;
    };
    packages.x86_64-linux.win64 = craneLib.buildPackage {
      name = cargo.name;
      version = cargo.version;
      src = craneLib.cleanCargoSource ./.;
      strictDeps = true;
      depsBuildBuild = [
        mingw.stdenv.cc
      ];
      nativeBuildInputs = [
        mingw.stdenv.cc
      ];
      buildInputs = [
        mingw.windows.pthreads
      ];
      CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
      doCheck = false;
      CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "x86_64-w64-mingw32-gcc";
    };
    packages.x86_64-linux.default = inputs.self.packages.x86_64-linux.eframe-app;
    devShells.x86_64-linux.default = pkgs.mkShell {
      shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${ldLibPath}"
        export WINIT_UNIX_BACKEND=x11
        export WAYLAND_DISPLAY=""
        export DISPLAY=:0
        export MESA_LOADER_DRIVER_OVERRIDE=llvmpipe
        export LIBGL_ALWAYS_SOFTWARE=1
        echo "✅ X11 WSL2 software rendering enabled"
      '';
      nativeBuildInputs = [ 
        toolchain
        pkgs.rust-analyzer
        # x11 WSL2 software rendering
        pkgs.libx11
        pkgs.libxcursor
        pkgs.libxrandr
        pkgs.libxi
        pkgs.libxext
        pkgs.libxkbcommon
        pkgs.libGL
        pkgs.mesa
        pkgs.vulkan-loader
      ];
    };
  };
}
