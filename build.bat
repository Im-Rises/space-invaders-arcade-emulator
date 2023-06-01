@echo off
wasm-pack build --target web --out-dir si-web-static/si-emu-pkg --release
wasm-pack build --target web --out-dir si-web-react/si-emu-pkg --release
cd si-web-react
npm install
yarn install
cd ..
pause
