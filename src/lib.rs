#![crate_name = "logfmt_logger"]

use log::{set_boxed_logger, set_max_level, Log, Metadata, Record, SetLoggerError};

use crate::filter::{create_default_filter, Filter};
use crate::fmt::{create_default_format, Format};

pub mod filter;
pub mod fmt;

/// Sets global logger to default instance of `LogFmtLogger`
pub fn init() {
  LogFmtLogger::default().init().unwrap();
}

pub struct LogFmtLogger {
  format: Box<dyn Format + Send + Sync>,
  filter: Box<dyn Filter + Send + Sync>,
}

impl LogFmtLogger {
  /// Returns a `LogFmtLogger` instance with default configuration
  pub fn new() -> Self {
    LogFmtLogger::default()
  }

  /// Returns a `LogFmtLogger` instance with supplied filter
  ///
  /// # Arguments
  /// * `filter` - The log record filter.
  /// # Examples
  /// ```
  /// use logfmt_logger::LogFmtLogger;
  /// use logfmt_logger::filter::env::EnvFilter;
  ///
  /// let logger = LogFmtLogger::new()
  ///   .with_filter(EnvFilter::new("MY_ENV_NAME"));
  /// ```
  pub fn with_filter<F: 'static>(mut self, filter: F) -> Self
  where
    F: Filter + Send + Sync,
  {
    self.filter = Box::new(filter);
    self
  }

  /// Returns a `LogFmtLogger` instance with supplied formatter
  ///
  /// # Arguments
  /// * `format` - Log the log record formatter.
  /// # Examples
  /// ```
  /// use logfmt_logger::LogFmtLogger;
  /// use logfmt_logger::fmt::color::ColorFormat;
  ///
  /// let logger = LogFmtLogger::new()
  ///   .with_format(ColorFormat::new());
  /// ```
  pub fn with_format<F: 'static>(mut self, format: F) -> Self
  where
    F: Format + Send + Sync,
  {
    self.format = Box::new(format);
    self
  }

  /// Sets global logger to this.
  ///
  /// # Errors
  ///
  /// An error is returned if a logger has already been set.
  ///
  pub fn init(self) -> Result<(), SetLoggerError> {
    set_max_level(self.filter.filter());
    set_boxed_logger(Box::new(self))
  }
}

impl Log for LogFmtLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    self.filter.enabled(metadata)
  }

  fn log(&self, record: &Record) {
    if !self.filter.matches(record) {
      return;
    }

    self.format.write(record).unwrap()
  }

  fn flush(&self) {}
}

impl Default for LogFmtLogger {
  fn default() -> Self {
    Self {
      filter: create_default_filter(),
      format: create_default_format(),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::LogFmtLogger;

  #[test]
  fn test_thing() {
    LogFmtLogger::new()
      .with_filter(crate::filter::nop::NopFilter::default())
      .init()
      .unwrap();

    log::info!("test");
    log::warn!("test");
    log::error!("test");
    log::debug!("test");
    log::trace!("test");
  }
}
