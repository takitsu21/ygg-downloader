install:
	cargo install --path .
build:
	cargo build
run: build
	cargo run