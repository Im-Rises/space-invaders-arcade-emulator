#!/bin/bash
wasm-pack build --target web --out-dir si-web-static/si-emu-pkg --release
wasm-pack build --target web --out-dir si-web-react/src/si-emu-pkg --release
