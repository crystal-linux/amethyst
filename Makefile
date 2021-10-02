debug:
	cargo build
	ln -sf target/debug/ame .
release:
	cargo build --release
	ln -sf target/release/ame .
clean:
	rm -rf target/ Cargo.lock ame
install:
	cargo build --release
	sudo cp target/release/ame /usr/bin/ame
