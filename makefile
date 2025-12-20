all: build

build:
	cargo build --release

install: build
	rm ~/.local/bin/tx
	ln -s $(shell pwd)/target/release/tx ~/.local/bin/tx

.PHONY: build install
