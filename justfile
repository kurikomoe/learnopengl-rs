build:
    cargo build

release:
    cargo build --release

test: build
    cargo test -- --test-threads=1