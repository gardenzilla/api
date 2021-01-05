 
.PHONY: release, test, dev

release:
	cargo update
	cargo build --release
	cargo test
	strip target/release/api

build:
	cargo update
	cargo build
	cargo test

dev:
	# . ./ENV.sh; backper
	cargo run;

test:
	cargo test