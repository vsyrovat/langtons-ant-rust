.PHONY: default fmt clean build release test

default: build

fmt:
	cargo fmt

clean:
	rm -rf target

build:
	cargo build

release:
	cargo build --release

test:
	cargo fmt --check
	cargo test
