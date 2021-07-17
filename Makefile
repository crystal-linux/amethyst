debug:
	cargo build
release:
	cargo build --release
clean:
	rm -rf target/ Cargo.lock
