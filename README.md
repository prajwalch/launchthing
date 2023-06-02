# launchthing

launchthing is a linux application launcher written in Rust using GTK4.

https://github.com/PrajwalCH/launchthing/assets/42384293/3617055e-48a9-4d8a-b50f-0bf3cb9162ee

## Install

Prebuilt binaries are not available for now as the project is in early development phase.
Consider [building it from source](#building-from-source).

## Building from source

To build the project you must have stable [rust v1.69.0+ installed](https://www.rust-lang.org/tools/install)
(plus [dependencies](#dependencies)). Once you have all the necessary things installed you can clone the source code and start to build it.

```sh
$ git clone https://github.com/PrajwalCH/launchthing
$ cd launchthing
$ cargo build --release
$ cargo run
```

### Dependencies

The only dependency you need is GTK4 library which you can install from the package manager provided by your Linux
distro.

#### Debian/Ubuntu

```sh
sudo apt install libgtk-4-dev build-essential
```

#### Arch Linux

```sh
sudo pacman -S gtk4 base-devel
```

#### Fedora

```sh
sudo dnf install gtk4-devel gcc
```
