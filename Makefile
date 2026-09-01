.PHONY: build run

build:
	cargo build
	cp target/debug/libn0.so target/debug/n0.node

run: build
	node src/node/main.js
