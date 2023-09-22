.PHONY: default fmt clean build release

default: build

fmt:
	cargo fmt

clean:
	rm -rf target

build:
	cargo build

release:
	cargo build --release
