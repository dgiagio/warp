all:
	$(MAKE) build

build:
	cargo build -p warp-runner --release --target x86_64-unknown-linux-gnu
	strip target/x86_64-unknown-linux-gnu/release/warp-runner

	CC=x86_64-apple-darwin15-clang cargo build -p warp-runner --release --target x86_64-apple-darwin
	x86_64-apple-darwin15-strip target/x86_64-apple-darwin/release/warp-runner

	cargo build -p warp-runner --release --target x86_64-pc-windows-gnu
	strip target/x86_64-pc-windows-gnu/release/warp-runner.exe

	cargo build -p warp-packer --release --target x86_64-unknown-linux-gnu
	strip target/x86_64-unknown-linux-gnu/release/warp-packer

	CC=x86_64-apple-darwin15-clang cargo build -p warp-packer --release --target x86_64-apple-darwin
	x86_64-apple-darwin15-strip target/x86_64-apple-darwin/release/warp-packer

	cargo build -p warp-packer --release --target x86_64-pc-windows-gnu
	strip target/x86_64-pc-windows-gnu/release/warp-packer.exe

clean:
	cargo clean

check:
	$(MAKE) build
	$(MAKE) test

test:
	cargo  test

.PHONY: all build clean check test
