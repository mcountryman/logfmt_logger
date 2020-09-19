<h1 align="center">logfmt_logger</h1>
<p align="center">
  <a href="https://docs.rs/logfmt_logger">
    <img src="https://docs.rs/logfmt_logger/badge.svg" alt="docs.rs" width="73" height="20"></img>
  </a>
  <a href="https://crates.io/crates/logfmt_logger">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/logfmt_logger" width="102" height="20"></img>
  </a>
</p>

Logger based off of env_logger using Heroku's [logfmt](https://brandur.org/logfmt) as an
output format.

### Preview
![terminal](docs/terminal.png)

### Quick Start
> `Cargo.toml`
```toml
logfmt_logger = "0.0.1"
```

> `src/main.rs`
```rust
use std::env;
use logfmt_logger;

fn main() {
  env::set_var("RUST_LOG", "trace");
  logfmt_logger::init();
}
```
