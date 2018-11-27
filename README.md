# eth_checksum

> Ethereum address checksum CLI in Rust

[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/miguelmota/eth_checksum/master/LICENSE) [![Build status](https://travis-ci.org/miguelmota/eth_checksum.svg)](https://travis-ci.org/miguelmota/eth_checksum) [![Crates.io](https://img.shields.io/crates/v/eth_checksum.svg)](https://crates.io/crates/eth_checksum)

## Install

```bash
cargo install eth_checksum
```

## Getting started

```bash
$ eth_checksum {address}
```

### Examples

```bash
$ eth_checksum 0xe0fc04fa2d34a66b779fd5cee748268032a146c0

0xe0FC04FA2d34a66B779fd5CEe748268032a146c0
```

```bash
$ eth_checksum 0xE0FC04FA2D34A66B779FD5CEE748268032A146C0

0xe0FC04FA2d34a66B779fd5CEe748268032a146c0
```

# License

[MIT](LICENSE)
