build:
  cargo run -p proto-build

check:
  cargo check -p neutron-proto --no-default-features --target wasm32-unknown-unknown

test:
  cargo test -p neutron-proto

lint:
  cargo +nightly clippy -p proto-build

fmt:
  cargo +nightly fmt
