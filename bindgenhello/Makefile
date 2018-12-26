run: build
	python3 -m http.server

build: webpack.config.js package.json
	 npm run build-debug

clean:
	cargo clean
	rm -f *.wasm *.ts bindgenhello.js Cargo.lock
