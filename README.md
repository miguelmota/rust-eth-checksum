# eth_checksum

> Ethereum address checksum library and CLI in Rust

[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/miguelmota/rust-eth-checksum/master/LICENSE) [![Build status](https://travis-ci.org/miguelmota/rust-eth-checksum.svg)](https://travis-ci.org/miguelmota/rust-eth-checksum) [![Crates.io](https://img.shields.io/crates/v/eth-checksum.svg)](https://crates.io/crates/eth-checksum)

## Install

```bash
cargo install eth_checksum
```

## Getting started

Using library:

```rust
extern crate eth_checksum;

fn main() {
    let addr = "0xe0fc04fa2d34a66b779fd5cee748268032a146c0";
    let checksummed = eth_checksum::checksum(&addr);

    println!("{}", checksummed);
    // 0xe0FC04FA2d34a66B779fd5CEe748268032a146c0
}
```

## CLI

```bash
$ eth_checksum {address}
```

Example:

```bash
$ eth_checksum 0xe0fc04fa2d34a66b779fd5cee748268032a146c0

0xe0FC04FA2d34a66B779fd5CEe748268032a146c0
```

Another Example:

```bash
$ eth_checksum 0xE0FC04FA2D34A66B779FD5CEE748268032A146C0

0xe0FC04FA2d34a66B779fd5CEe748268032a146c0
```

## Test

```bash
make test
```

## License

[MIT](LICENSE)
