.PHONY: build
build:
	cargo component build --release --manifest-path moderator/Cargo.toml
	cargo build --manifest-path api/Cargo.toml --target wasm32-wasip1 --release