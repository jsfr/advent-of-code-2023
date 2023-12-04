upgrade:
	cargo update
	cargo upgrade

lint:
	cargo clippy --all-targets --all-features -- -W clippy::pedantic

fmt:
    cargo fmt

fix:
    cargo clippy --all-targets --all-features --fix --allow-staged -- -W clippy::pedantic

test:
	cargo test
