.PHONY: build clean

build:
	wasm-pack build --target web --out-dir web/pkg --release

clean:
	rm -rf web/pkg
