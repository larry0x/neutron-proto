# neutron-proto

Protobuf type definitions in Rust for the Neutron blockchain.

Based on the [proto-build](https://github.com/cosmos/cosmos-rust/blob/main/proto-build/src/main.rs) script originally created by Justin Kilpatrick, Tony Arcieri, and others.

## How to use

To use the `neutron-proto` crate in a CosmWasm contract, make sure to turn off the gRPC-related features, which are not wasm-compatible:

```toml
[dependencies]
neutron-proto = { git = "github.com/larry0x/neutron-proto", rev = "...", default-features = false }
```

## License

Contents in this repository are released under the [Apache-2.0](../LICENSE) license.
