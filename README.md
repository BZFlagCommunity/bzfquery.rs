# bzfquery.rs

[![Crates.io](https://img.shields.io/crates/v/bzfquery)](https://crates.io/crates/bzfquery)
[![GitHub release](https://img.shields.io/github/v/release/BZFlagCommunity/bzfquery.rs?include_prereleases&sort=semver)](https://github.com/BZFlagCommunity/bzfquery.rs/releases)
[![GitHub license](https://img.shields.io/github/license/BZFlagCommunity/bzfquery.rs)](LICENSE)
[![CI](https://github.com/BZFlagCommunity/bzfquery.rs/workflows/CI/badge.svg)](https://github.com/BZFlagCommunity/bzfquery.rs/actions?query=workflow%3ACI)
[![Docs](https://img.shields.io/badge/docs-docs.rs-blue)](https://docs.rs/bzfquery)

Rust version of bzfquery with no external dependencies. It can be used as a library or from the command line.

# CLI

```sh
cargo install bzfquery
```

Usage: `bzfquery host[:port]`

# Library

```rust
use bzfquery::*;

let query = query("bzflag.allejo.io", 5130);
println!("{}", query);
```
