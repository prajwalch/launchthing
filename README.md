# launchthing

launchthing is a linux application launcher written in Rust using GTK4.

## Screenshots

| ![ss][0] | ![ss][1] |
|----------|----------|
| ![ss][2] | ![ss][3] |

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

[//]: # (ImageLinks)

[//]: # (@formatter:off)
[0]: https://github.com/PrajwalCH/launchthing/assets/42384293/8193e954-0f86-4796-9e5c-f8ca5ca11c06
[1]: https://github.com/PrajwalCH/launchthing/assets/42384293/fcfe956f-ca97-4e83-bda1-b8baffad1606
[2]: https://github.com/PrajwalCH/launchthing/assets/42384293/43719c03-77cc-4ba6-bb3a-0d1c29ed1a23
[3]: https://github.com/PrajwalCH/launchthing/assets/42384293/b54cc67a-84ac-43fe-9ede-e5c652d5c9c7
[//]: # (@formatter:on)
