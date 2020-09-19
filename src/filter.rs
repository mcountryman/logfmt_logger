use log::{LevelFilter, Metadata, Record};

pub trait Filter {
  fn filter(&self) -> LevelFilter;
  fn enabled(&self, metadata: &Metadata) -> bool;
  fn matches(&self, record: &Record) -> bool;
}

pub fn create_default_filter() -> Box<dyn Filter + Send + Sync> {
  #[cfg(feature = "env")]
  return Box::new(env::EnvFilter::default());
  #[cfg(not(feature = "env"))]
  return Box::new(nop::NopFilter::default());
}

#[cfg(feature = "env")]
pub mod env {
  use env_logger::filter::{Builder as EnvLogBuilder, Filter as EnvLogFilter};
  use log::{LevelFilter, Metadata, Record, SetLoggerError};

  use crate::filter::Filter;
  use crate::LogFmtLogger;

  pub struct EnvFilter {
    inner: EnvLogFilter,
  }

  impl EnvFilter {
    pub fn new(env: &str) -> Self {
      Self {
        inner: EnvLogBuilder::from_env(env).build(),
      }
    }
  }

  impl Filter for EnvFilter {
    fn filter(&self) -> LevelFilter {
      self.inner.filter()
    }
    fn enabled(&self, metadata: &Metadata) -> bool {
      self.inner.enabled(metadata)
    }
    fn matches(&self, record: &Record) -> bool {
      self.inner.matches(record)
    }
  }

  impl Default for EnvFilter {
    fn default() -> Self {
      Self {
        inner: EnvLogBuilder::from_env("RUST_LOG").build(),
      }
    }
  }

  pub trait WithEnvFilter {
    fn init(env: &str) -> Result<(), SetLoggerError>;
    fn with_env_filter(self, env: &str) -> Self;
  }

  impl WithEnvFilter for LogFmtLogger {
    fn init(env: &str) -> Result<(), SetLoggerError> {
      LogFmtLogger::new().with_env_filter(env).init()
    }

    fn with_env_filter(self, env: &str) -> Self {
      self.with_filter(EnvFilter::new(env))
    }
  }
}

pub mod nop {
  use log::{LevelFilter, Metadata, Record};

  use crate::filter::Filter;

  #[derive(Default)]
  pub struct NopFilter;

  impl Filter for NopFilter {
    fn filter(&self) -> LevelFilter {
      LevelFilter::Trace
    }
    fn enabled(&self, _metadata: &Metadata) -> bool {
      true
    }
    fn matches(&self, _record: &Record) -> bool {
      true
    }
  }
}
