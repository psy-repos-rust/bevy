# Installing Linux dependencies

This page lists the required dependencies to build a Bevy project on your Linux machine.

If you don't see your distro present in the list, feel free to add the instructions in this document.

## [Ubuntu](https://ubuntu.com/)

```bash
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
```

if using Wayland, you will also need to install

```bash
sudo apt-get install libwayland-dev libxkbcommon-dev
```

Depending on your graphics card, you may have to install one of the following:
`vulkan-radeon`, `vulkan-intel`, or `mesa-vulkan-drivers`

Compiling with clang is also possible - replace the `g++` package with `clang`.

## Windows Subsystem for Linux (WSL 2)

Up-to-date WSL Installs for Windows 10 & 11 include WSLg, which provides
necessary servers for passing graphics and audio between Windows and the WSL instance.
With WSLg, a user's WSL instance can use X11 as well as Wayland.
For more information, see WSLg [documentation](https://github.com/microsoft/wslg#wslg-architecture-overview).

Prior to the release of [WSL Gui (WSLg)](https://en.wikipedia.org/wiki/Windows_Subsystem_for_Linux#WSLg)
around 4/20/2021, users had to [manually set up servers](https://wiki.ubuntu.com/WSL#Advanced_Topics) on windows for graphic and audio.
Make note of the date for documentation found across the internet.
Following advice from before WSLg's release can lead to additional conflicts.

## [Fedora](https://getfedora.org/)

```bash
sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel
```

if using Wayland, you will also need to install

```bash
sudo dnf install wayland-devel libxkbcommon-devel
```

If there are errors with linking during the build process such as:

```bash
 = note: /usr/bin/ld: skipping incompatible /usr/lib/libasound.so when searching for -lasound
          /usr/bin/ld: skipping incompatible /usr/lib/libasound.so when searching for -lasound
          /usr/bin/ld: skipping incompatible /usr/lib/gcc/x86_64-redhat-linux/10/../../../libasound.so when searching for -lasound
          /usr/bin/ld: skipping incompatible /lib/libasound.so when searching for -lasound
          /usr/bin/ld: skipping incompatible /usr/lib/libasound.so when searching for -lasound
          /usr/bin/ld: cannot find -lasound
```

Add your arch to the end of the package to remove the linker error. For example:

```bash
sudo dnf install alsa-lib-devel.x86_64
```

Or if there are errors such as:

```txt
  --- stderr
  thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "`\"pkg-config\" \"--libs\" \"--cflags\" \"libudev\"` did not exit successfully: exit status: 1\n--- stderr\nPackage libudev was not found in the pkg-config search path.\nPerhaps you should add the directory containing `libudev.pc'\nto the PKG_CONFIG_PATH environment variable\nNo package 'libudev' found\n"', /home/<user>/.cargo/registry/src/github.com-1ecc6299db9ec823/libudev-sys-0.1.4/build.rs:38:41
  stack backtrace:
     0: rust_begin_unwind
               at /rustc/9bb77da74dac4768489127d21e32db19b59ada5b/library/std/src/panicking.rs:517:5
     1: core::panicking::panic_fmt
               at /rustc/9bb77da74dac4768489127d21e32db19b59ada5b/library/core/src/panicking.rs:96:14
     2: core::result::unwrap_failed
               at /rustc/9bb77da74dac4768489127d21e32db19b59ada5b/library/core/src/result.rs:1617:5
     3: core::result::Result<T,E>::unwrap
               at /rustc/9bb77da74dac4768489127d21e32db19b59ada5b/library/core/src/result.rs:1299:23
     4: build_script_build::main
               at ./build.rs:38:5
     5: core::ops::function::FnOnce::call_once
               at /rustc/9bb77da74dac4768489127d21e32db19b59ada5b/library/core/src/ops/function.rs:227:5
  note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
warning: build failed, waiting for other jobs to finish...
error: build failed
```

Set the `PKG_CONFIG_PATH` env var to `/usr/lib/<target>/pkgconfig/`. For example on an x86_64 system:

```txt
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig/"
```

## [Arch](https://archlinux.org/) / [Manjaro](https://manjaro.org/)

```bash
sudo pacman -S libx11 pkgconf alsa-lib libxcursor libxrandr libxi
```

Install `pipewire-alsa` or `pulseaudio-alsa` depending on the sound server you are using.

Depending on your graphics card, you may have to install one of the following:
`vulkan-radeon`, `vulkan-intel`, or `mesa-vulkan-drivers`

## [Void](https://voidlinux.org/)

```bash
sudo xbps-install -S pkgconf alsa-lib-devel libX11-devel eudev-libudev-devel
```

## [Nix](https://nixos.org)

### flake.nix

Add a `flake.nix` file to the root of your GitHub repository containing:

```nix
{
  description = "bevy flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs =
              [
                # Rust dependencies
                (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })
                pkg-config
              ]
              ++ lib.optionals (lib.strings.hasInfix "linux" system) [
                # for Linux
                # Audio (Linux only)
                alsa-lib
                # Cross Platform 3D Graphics API
                vulkan-loader
                # For debugging around vulkan
                vulkan-tools
                # Other dependencies
                libudev-zero
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
                libxkbcommon
              ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              vulkan-loader
              xorg.libX11
              xorg.libXi
              xorg.libXcursor
              libxkbcommon
            ];
          };
      }
    );
}
```

> [!TIP]
> We have confirmed that this flake.nix can be used successfully on NixOS and MacOS with Rust's edition set to 2021.

### shell.nix

Add a `shell.nix` file to the root of the project containing:

```nix
{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    udev alsa-lib-with-plugins vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
```

And enter it by just running `nix-shell`.
You should be able compile Bevy programs using `cargo run` within this nix-shell.
You can do this in one line with `nix-shell --run "cargo run"`.

If running nix on a non NixOS system (such as ubuntu, arch etc.), [NixGL](https://github.com/nix-community/nixGL) is additionally required,
to link graphics drivers into the context of software installed by nix:

1. Install a system specific nixGL wrapper ([docs](https://github.com/nix-community/nixGL)).
   - If you're running a nvidia GPU choose `nixVulkanNvidia`.
   - Otherwise, choose another wrapper appropriate for your system.
2. Run `nixVulkanNvidia-xxx.xxx.xx cargo run` to compile a bevy program, where `xxx-xxx-xx` denotes the graphics driver version `nixVulkanNvidia` was compiled with.

This is also possible with [Nix flakes](https://nixos.org/manual/nix/unstable/command-ref/new-cli/nix3-flake.html).
Instead of creating `shell.nix`, you just need to add the derivation (`mkShell`)
to your `devShells` in `flake.nix`. Run `nix develop` to enter the shell or
`nix develop -c cargo run` to just run the program. See
[Nix's documentation](https://nixos.org/manual/nix/unstable/command-ref/new-cli/nix3-develop.html)
for more information about `devShells`.

Note that this template does not add Rust to the environment because there are many ways to do it.
For example, to use stable Rust from nixpkgs, you can add `cargo` and `rustc` to `nativeBuildInputs`.

[Here](https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/ju/jumpy/package.nix)
is an example of packaging a Bevy program in nix.

## [OpenSUSE](https://www.opensuse.org/)

```bash
   sudo zypper install libudev-devel gcc-c++ alsa-lib-devel
```

## [Gentoo](https://www.gentoo.org/)

```bash
   sudo emerge --ask libX11 pkgconf alsa-lib
```

When using an AMD Radeon GPU, you may also need to emerge `amdgpu-pro-vulkan` to get Bevy to find the GPU.

When using a NVIDIA GPU with the proprietary driver (eg. `x11-drivers/nvidia-drivers`), you may also need to emerge `media-libs/vulkan-loader` to get Bevy to find the GPU. NVIDIA Vulkan driver is included in `nvidia-driver`, but may need the loader to find the correct driver. See Gentoo [Documentation](https://wiki.gentoo.org/wiki/Vulkan) for details.

## [Clear Linux OS](https://clearlinux.org/)

```bash
sudo swupd bundle-add devpkg-alsa-lib
sudo swupd bundle-add devpkg-libgudev
```

## [Alpine Linux](https://alpinelinux.org/)

Run the following command to install `GNU C compiler, standard C development libraries, pkg-config, X11 development libraries, ALSA development libraries, eudev development libraries`:

```sh
sudo apk add gcc libc-dev pkgconf libx11-dev alsa-lib-dev eudev-dev
```

Install a GPU renderer for you graphics card. For Intel integrated GPUs:

```sh
sudo apk add mesa-vulkan-intel
```

If you have issues with `winit` such as `Failed to initialize backend!` or similar, try adding the following to your `~/.cargo/config.toml` (more information at the [issue #1818](https://github.com/rust-windowing/winit/issues/1818) of the [winit repository](https://github.com/rust-windowing/winit):

```toml
[build]
rustflags = ["-C", "target-feature=-crt-static"]
```

## [Solus](https://getsol.us)

```sh
sudo eopkg it -c system.devel
sudo eopkg it g++ libx11-devel alsa-lib-devel
```

If using Wayland, you may also need to install

```sh
sudo eopkg it wayland-devel libxkbcommon-devel
```

Compiling with clang is also possible - replace the `g++` package with `llvm-clang`

## [FreeBSD](https://www.freebsd.org/)

It is necessary to have the hgame module loaded in order to satisfy gli-rs. It will still throw an error, but the program should run successfully. You can make sure the kernel module is loaded on start up by adding the following line to /boot/loader.conf:

```sh
hgame_load="YES"
```
