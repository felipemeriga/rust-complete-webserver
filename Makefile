prog :=xnixperms

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

format:
	cargo fmt
lint:
	cargo clippy
build:
	cargo build $(release)

run:
	cargo run

test:
	cargo test -- --test-threads=1

all: build run

help:
	@echo "usage: make $(prog) [debug=1]"