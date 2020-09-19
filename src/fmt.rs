use std::io::Error;

use log::Record;

pub trait Format {
  fn write(&self, record: &Record) -> Result<(), Error>;
}

pub fn create_default_format() -> Box<dyn Format + Send + Sync> {
  #[cfg(feature = "color")]
  return Box::new(color::ColorFormat::default());
  #[cfg(not(feature = "color"))]
  return Box::new(default::DefaultFormat::default());
}

pub mod default {
  use std::io::Error;

  use log::Record;

  use crate::fmt::Format;

  #[derive(Default)]
  pub struct DefaultFormat;

  impl Format for DefaultFormat {
    fn write(&self, record: &Record) -> Result<(), Error> {
      let level = record.level();
      let level_name = level.to_string().to_lowercase();

      print!("level={}", level_name);
      print!(" message=\"{}\"", record.args());
      print!(" target=\"{}\"", record.target());
      println!();

      Ok(())
    }
  }
}

#[cfg(feature = "color")]
pub mod color {
  use std::collections::HashMap;
  use std::io::{Error, Write};

  use log::{Level, Record};
  use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

  use crate::fmt::Format;

  pub struct ColorFormat {
    levels: HashMap<Level, Color>,
  }

  impl ColorFormat {
    pub fn new() -> Self {
      Self {
        levels: vec![
          (Level::Error, Color::Red),
          (Level::Trace, Color::Magenta),
          (Level::Debug, Color::Cyan),
          (Level::Info, Color::Blue),
          (Level::Warn, Color::Yellow),
        ]
        .into_iter()
        .collect(),
      }
    }
  }

  #[cfg(debug_assertions)]
  impl Format for ColorFormat {
    fn write(&self, record: &Record) -> Result<(), Error> {
      let mut stdout = StandardStream::stdout(ColorChoice::Always);

      let level = record.level();
      let level_name = level.to_string().to_lowercase();
      let level_color = self.levels.get(&level).cloned();

      stdout.set_color(ColorSpec::new().set_fg(level_color))?;

      write!(&mut stdout, "[{}]", level_name)?;

      if level > Level::Error {
        stdout.set_color(
          ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_intense(true),
        )?;
      }

      write!(&mut stdout, " {}", record.args())?;

      stdout.set_color(
        ColorSpec::new()
          .set_fg(Some(Color::White))
          .set_intense(false),
      )?;

      write!(&mut stdout, " target=\"{}\"", record.target())?;
      write!(&mut stdout, " message=\"{}\"", record.args())?;
      writeln!(&mut stdout)?;

      stdout.flush()
    }
  }

  impl Default for ColorFormat {
    fn default() -> Self {
      ColorFormat::new()
    }
  }
}
