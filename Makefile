src = $(wildcard src/*.rs) $(wildcard tests/*.rs)

target/debug/errfmt: $(src)
	cargo test
	cargo build

test:
	cargo test

install:
	cargo install --path . --force

doc:
	cargo doc --open
