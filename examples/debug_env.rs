#[macro_use]
extern crate log;

use std::env;

fn main() {
  env::set_var("RUST_LOG", "trace");
  logfmt_logger::init();

  info!("Info message");
  warn!("Warn message");
  debug!("Debug message");
  trace!("Trace message");
  error!("Error message");
}
