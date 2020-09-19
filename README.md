# logfmt_logger

Logger based off of env_logger using Heroku's [logfmt](https://brandur.org/logfmt) as an
output format.

### Quick Start
> `Cargo.toml`
```toml
logfmt_logger = "0.0.1"
```

> `src/main.rs`
```rust
use logfmt_logger;

fn main() {
  logfmt_logger::init();
}
```

![terminal](docs/terminal.png)