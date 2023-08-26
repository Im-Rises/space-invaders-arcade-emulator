#!/bin/bash
wasm-pack build --target web --out-dir si-web-static/si-emu-pkg --release
cd - || exit
