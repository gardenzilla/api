include ../ENV.list
export $(shell sed 's/=.*//' ../ENV.list)

.PHONY: release, test, dev, run, run_release

release:
	cargo update
	cargo build --release
	strip target/release/api

build:
	cargo update
	cargo build
	cargo test

run:
	cargo run

run_release:
	cargo run --release

dev:
	# . ./ENV.sh; backper
	cargo run

test:
	cargo test