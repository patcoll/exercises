all: check clippy test

clean:
	rm -rf ./target

check:
	cargo check

clippy:
	cargo clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms

test:
	cargo test

watch:
	cargo watch -x check -x 'clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms' -x 'test -- --nocapture'
