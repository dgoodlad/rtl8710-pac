.PHONY: svd2rust
svd2rust:
	rm -rf src
	svd2rust -i rtl8710.svd
	form -i lib.rs -o src/
	rm lib.rs
	cargo fmt
