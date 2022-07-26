# space_invaders_arcade_emulator

Space Invaders arcade game emulator in development made in Rust.

## Description

## Features

## Images

## Videos

## Quick start

## Compilation

First thing you need is to install cargo. You have it if you already have installed Rust, if not, please follow the
instruction at the link below:
<https://www.rust-lang.org/tools/install>

With Rust, you can compile the project in two-way, debug and release. To compile go to the project root folder and type
one of the two following commands below (If you want to use the emulator please compile using the second command).

```bash
cargo build
```

or

```bash
cargo build --release
```

The compiled app will be in the folder `target/debug` or `target/release` depending on the compilation you did.

## Rust tests

<https://doc.rust-lang.org/book/ch11-00-testing.html>

```bash
cargo test
```

If you want some debug infos about the cpu type:

```bash
cargo test -- --nocapture
```

## GitHub Actions

[![Rust](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust.yml)
[![rust-clippy analyze](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-clippy.yml)

The project is set with a set of different scripts:

- Rust : Check the code compilation.
- Rust clippy : Evaluate the code quality (error, warnings, etc...).
- Rust publisher : Publish the app to releases when pushing to the main branch.

## Documentation

emulator101:  
<http://www.emulator101.com>

Computer Archeology:  
<https://www.computerarcheology.com/Arcade/SpaceInvaders/Hardware.html>

Emudev.de:  
<https://emudev.de/q00-si/a-short-fun-project/>

Rust:  
<https://doc.rust-lang.org/book>

Intel 8080 documentations:  
<https://pastraiser.com/cpu/i8080/i8080_opcodes.html>  
<https://archive.org/details/8080Datasheet>  
<https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf>

Wikipedia:  
<https://en.wikipedia.org/wiki/Intel_8080>
