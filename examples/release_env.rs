#[macro_use]
extern crate log;

use logfmt_logger::filter::nop::NopFilter;
use logfmt_logger::fmt::default::DefaultFormat;
use logfmt_logger::LogFmtLogger;

fn main() {
  LogFmtLogger::new()
    .with_filter(NopFilter::default())
    .with_format(DefaultFormat::default())
    .init()
    .unwrap();

  info!("Info message");
  warn!("Warn message");
  debug!("Debug message");
  trace!("Trace message");
  error!("Error message");
}
