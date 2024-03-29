# space-invaders-arcade-emulator

<p align="center">
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="rustLogo" style="height:60px;"/>
    <img src="https://img.shields.io/badge/JavaScript-323330?style=for-the-badge&logo=javascript&logoColor=F7DF1E" alt="javascriptLogo" style="height:50px;">
    <img src="https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB" alt="reactLogo" style="height:50px;">
    <img src="https://img.shields.io/badge/Sass-CC6699?style=for-the-badge&logo=sass&logoColor=white" alt="scssLogo" style="height:50px;">
    <img src="https://img.shields.io/badge/CSS-239120?&style=for-the-badge&logo=css3&logoColor=white" alt="cssLogo" style="height:50px;">
</p>

## Description

Space Invaders arcade game emulator written in Rust for the web.

Complete Emulator of the Intel 8080, the app is implemented to run the Space Invaders Arcade game.

## 🚀🚀[You can try it online from your browser](https://im-rises.github.io/space-invaders-arcade-emulator-website/) 🚀🚀

The demo source code is available [here](https://github.com/Im-Rises/space-invaders-arcade-emulator-website) 🚀🚀

The game is implemented with all the 10 sounds of the original game and can be played in black and white or in color (
SV), normal screen mode (TV) or in colored mode (CV).

# 🚀🚀 [The package is available on npm](https://www.npmjs.com/package/space-invaders-arcade-emulator) 🚀🚀

## Images

### Without background

|                                                         SV version                                                          |                                                           TV version                                                           |                                                           CV version                                                           |
|:---------------------------------------------------------------------------------------------------------------------------:|:------------------------------------------------------------------------------------------------------------------------------:|:------------------------------------------------------------------------------------------------------------------------------:|
| ![NOBG_BW](https://github.com/Im-Rises/space-invaders-arcade-emulator/assets/59691442/96276b2a-d75c-4eef-ae76-74624960ba19) | ![NOBG_Color](https://github.com/Im-Rises/space-invaders-arcade-emulator/assets/59691442/32bc3e25-7883-4229-bab3-7bbfae745e9e) | ![NOBG_Geeen](https://github.com/Im-Rises/space-invaders-arcade-emulator/assets/59691442/4ec1df0d-98e5-4327-b25c-da80e6a44470) |

### Using background 1

| SV version                                                                                                                    | TV version                                                                                                                 | CV version                                                                                                                    |
|-------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| ![BG2_Green](https://github.com/Im-Rises/space-invaders-arcade-emulator/assets/59691442/433a9da2-cdb8-48f8-85b8-9e615766bdf9) | ![BG2_BW](https://github.com/Im-Rises/space-invaders-arcade-emulator/assets/59691442/7a5c9618-9fd6-48a5-81eb-81d5964aa98e) | ![BG2_Color](https://github.com/Im-Rises/space-invaders-arcade-emulator/assets/59691442/39adf0b0-2fe9-488d-a10f-b390c4c1141a) |

### Using background 2

| SV version                                                                                                                 | TV version                                                                                                                    | CV version                                                                                                                    |
|----------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| ![BG1_BW](https://github.com/Im-Rises/space-invaders-arcade-emulator/assets/59691442/d0a903f4-b92f-48a2-b4d2-f4173be2ce3d) | ![BG1_Green](https://github.com/Im-Rises/space-invaders-arcade-emulator/assets/59691442/7a31f2f5-38ba-4881-894c-bfacc2faf659) | ![BG1_Color](https://github.com/Im-Rises/space-invaders-arcade-emulator/assets/59691442/563ab206-1382-4311-b0b5-76d34b41a0aa) |

## Videos

https://user-images.githubusercontent.com/59691442/183045566-0a3df947-06e7-4c46-9fc6-9d2b8f7d9a46.mp4

## Controls

You can use the keyboard to play the game.

| Arcade buttons | Emulator/Keyboard |
|----------------|-------------------|
| Insert coin    | C                 |
| P1 start       | Space             |
| P1 shoot       | ↑                 |
| P1 ←           | ←                 |
| P1 →           | →                 |
| P2 start       | G                 |
| P2 shoot       | E                 |
| P2 ←           | S                 |
| P2 →           | F                 |

The original game is mapped with some inputs that allow the constructor to choose the difficulty. This allows you to
increase the numer of lives and change the extra ship necessity points. To change those settings check the checkboxes
before starting the game.

## GitHub Actions

[![rust](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust.yml)
[![rust-clippy analyze](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-clippy.yml)
[![rustfmt check](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rustfmt.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rustfmt.yml)
[![Wasm CI](https://github.com/Im-Rises/space-invaders-arcade-emulator/actions/workflows/wasm.yml/badge.svg)](https://github.com/Im-Rises/space-invaders-arcade-emulator/actions/workflows/wasm.yml)
[![Wasm NPM Publish](https://github.com/Im-Rises/space-invaders-arcade-emulator/actions/workflows/wasm-npm-publish.yml/badge.svg)](https://github.com/Im-Rises/space-invaders-arcade-emulator/actions/workflows/wasm-npm-publish.yml)

The project is set with a set of different scripts:

- Rust : Build the project in native mode and run the unit tests.
- rust-clippy analyze : Evaluate the code quality (error, warnings, etc...).
- rustfmt check :  Check the code good formatting
- Wasm CI : Build the project in wasm mode.
- Wasm NPM Publish : Build the project in wasm mode and publish it on npm.

## Documentation

emulator101:  
<http://www.emulator101.com>

Computer Archeology:  
<https://www.computerarcheology.com/Arcade/SpaceInvaders/Hardware.html>

Emudev.de:  
<https://emudev.de/q00-si/a-short-fun-project/>

Rust:  
<https://doc.rust-lang.org/book>

SDL2 Rust:  
<https://github.com/Rust-SDL2/rust-sdl2>

SDL2 libs download:  
<https://www.libsdl.org/download-2.0.php>  
<https://github.com/libsdl-org/SDL_mixer/releases>

rust-clippy:  
<https://github.com/rust-lang/rust-clippy>

rustfmt:  
<https://github.com/rust-lang/rustfmt>

Intel 8080 documentations:  
<https://archive.org/details/8080Datasheet>  
<https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf>
<http://bitsavers.org/components/intel/MCS80/9800301D_8080_8085_Assembly_Language_Programming_Manual_May81.pdf>

Intel 8080 opcodes table:  
<https://www.pastraiser.com/cpu/i8080/i8080_opcodes.html>

Wikipedia:  
<https://en.wikipedia.org/wiki/Intel_8080>

Test Roms for the Intel 8080:  
<https://github.com/superzazu/8080/>  
<https://altairclone.com/downloads/cpu_tests/>  
<http://www.emulator101.com/full-8080-emulation.html>

Space Invaders Audio files:  
<https://samples.mameworld.info>  
<https://www.classicgaming.cc/classics/space-invaders/sounds>

Mozzila Rust to WebAssembly:  
<https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm>

wasm-bindgen:  
<https://rustwasm.github.io/docs/wasm-bindgen/>

js-sys:  
<https://docs.rs/js-sys/latest/js_sys/>

web-sys:  
<https://docs.rs/web-sys/latest/web_sys/>

trcf.net:  
<https://tcrf.net/Space_Invaders_(Arcade)>

tobiasvl.github.io:  
<https://tobiasvl.github.io/blog/space-invaders/>

trcf.net:  
<https://tcrf.net/Space_Invaders_(Arcade)>

eab.abime.net:  
<https://eab.abime.net/showthread.php?t=105132&page=2>

ArcadeArtwork:  
<https://www.arcadeartwork.org/index.php?%2Fcategory%2F57>

codepen.io:  
<https://codepen.io/DanielWeiner/pen/naybVd>

## Contributors

Quentin MOREL :

- @Im-Rises
- <https://github.com/Im-Rises>

[![GitHub contributors](https://contrib.rocks/image?repo=Im-Rises/GameBoyEmulator)](https://github.com/Im-Rises/GameBoyEmulator/graphs/contributors)
