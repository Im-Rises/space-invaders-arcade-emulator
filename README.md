# space_invaders_arcade_emulator

<p align="center">
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="rustLogo" style="height:60px;"/>
    <img src="https://img.shields.io/badge/JavaScript-323330?style=for-the-badge&logo=javascript&logoColor=F7DF1E" alt="javascriptLogo" style="height:50px;">
    <img src="https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB" alt="reactLogo" style="height:50px;">
    <img src="https://img.shields.io/badge/Sass-CC6699?style=for-the-badge&logo=sass&logoColor=white" alt="scssLogo" style="height:50px;">
    <img src="https://img.shields.io/badge/CSS-239120?&style=for-the-badge&logo=css3&logoColor=white" alt="cssLogo" style="height:50px;">
</p>

## Description

Space Invaders' arcade game emulator written in Rust for the web.

Complete Emulator of the Intel 8080, the app is implemented to run the Space Invaders Arcade game.

The game is implemented with all the 10 sounds of the original game and can be played in black and white or in color (
SV), normal screen mode (TV) or in colored mode (CV).

## Images

### Using background

| SV version                                                                                                                                 | TV version                                                                                                                                 | CV version                                                                                                                                 |
|--------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| ![si_web_sv_mode](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/assets/59691442/7f2f9ab0-00ed-40d2-b0ec-0b754c8f3d47) | ![si_web_tv_mode](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/assets/59691442/c4e52a7a-02ac-4be3-812f-9e1c4119b378) | ![si_web_cv_mode](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/assets/59691442/b218fdad-6d72-478c-a127-d4d026a05d28) |

### Without background

| SV version                                                                                                                              | TV version                                                                                                                              | CV version                                                                                                                              |
|-----------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| ![si_no_bg_sv](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/assets/59691442/c6c386fd-6c4e-48af-8a79-f78a67fb5a66) | ![si_no_bg_tv](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/assets/59691442/9717f304-53c1-463e-aaa2-3fa170f378ab) | ![si_no_bg_cv](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/assets/59691442/dcc827b4-42ec-4fc3-b2f9-de9dfc397d94) |

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

The original game is mapped with some inputs that allow the constructor to choose the difficulty. I Mapped those inputs
to the keyboard. This allows you to increase the numer of lives and change the extra ship necessity points.

Before pressing start with player 1 or 2, you can choose the number of life you want to have for a game party.

| Emulator/Keyboard | Emulator buttons                   |
|-------------------|------------------------------------|
| K                 | 1 more life                        |
| L                 | 2 more lives                       |
| M                 | extra ship at 1000 instead of 1500 |

> **Note**  
> If you don't keep pressed K or L before pressing start and starting a new game you will have 3 lives.  
> In the same way, you can enable the extra ship to came at 1000 points instead of 1500, but you just need to press
> the button one time (a confirmation will be displayed in the console).

## GitHub Actions

[![rust](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust.yml)
[![rust-clippy analyze](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rust-clippy.yml)
[![rustfmt check](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rustfmt.yml/badge.svg?branch=main)](https://github.com/Im-Rises/space_invaders_arcade_emulator/actions/workflows/rustfmt.yml)
[![ESLint](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/actions/workflows/eslint.yml/badge.svg)](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/actions/workflows/eslint.yml)
[![Node.js Wasm CI](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/actions/workflows/node-wasm.yml/badge.svg)](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/actions/workflows/node-wasm.yml)
[![Node.js Wasm CI publish](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/actions/workflows/node-wasm-publish.yml/badge.svg)](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/actions/workflows/node-wasm-publish.yml)
[![pages-build-deployment](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/Im-Rises/space-invaders-arcade-emulator-website/actions/workflows/pages/pages-build-deployment)

The project is set with a set of different scripts:

- Rust : Build the project in native mode and run the unit tests.
- rust-clippy analyze : Evaluate the code quality (error, warnings, etc...).
- rustfmt check :  Check the code good formatting
- ESLint : Check the code good formatting
- Node.js Wasm CI : Build the WebAssembly file and the website.
- Node.js Wasm CI publish : Publish the website with the webassembly file on the gh-pages branch.
- pages-build-deployment : Publish the website form the gh-pages branch to GitHub Pages.

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

## Contributors

Quentin MOREL :

- @Im-Rises
- <https://github.com/Im-Rises>

[![GitHub contributors](https://contrib.rocks/image?repo=Im-Rises/GameBoyEmulator)](https://github.com/Im-Rises/GameBoyEmulator/graphs/contributors)
