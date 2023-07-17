build:
	cargo build

run:
	cargo run

dev:
	cargo watch -x run

# build for web
web-build:
	cargo b --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir dist/ --target web target/wasm32-unknown-unknown/release/fireworks.wasm

web-run: web-build
	basic-http-server dist

# start web server and rebuild for web on every change
web-dev:
	bash -c "basic-http-server dist &";
	cargo watch -- ~/.cargo/bin/just web-build

web-dev-kill:
	pkill -f "basic-http-server"
	
