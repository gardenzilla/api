include ../ENV.list
export $(shell sed 's/=.*//' ../ENV.list)

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

run:
	cargo run;

dev:
	# . ./ENV.sh; backper
	cargo run;

test:
	cargo test