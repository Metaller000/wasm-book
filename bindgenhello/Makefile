run: build
	npm install
	npm run serve

build: webpack.config.js package.json bindgenhello.js

bindgenhello.js: target/wasm32-unknown-unknown/debug/bindgenhello.wasm
	wasm-bindgen $< --out-dir .

target/wasm32-unknown-unknown/debug/bindgenhello.wasm: src/lib.rs Cargo.toml
	cargo +nightly build --target wasm32-unknown-unknown

clean:
	cargo clean
	rm -f *.wasm *.ts bindgenhello.js Cargo.lock package-lock.json