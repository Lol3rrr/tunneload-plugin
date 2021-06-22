//! This module contains a couple of logging related parts that can be used
//! to output logs in the Tunneload instance

use log::{Level, Metadata, Record, SetLoggerError};

use crate::raw;

struct TunnloadLogger {
    level: Level,
}

/// Initializes the Plugin-Logger with the given Log-Level
pub fn init(level: log::Level) -> Result<(), SetLoggerError> {
    let logger = Box::new(TunnloadLogger { level });
    log::set_logger(Box::leak(logger)).map(|()| log::set_max_level(log::LevelFilter::Info))
}

impl log::Log for TunnloadLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        error(&format!("{}", record.args()));
    }

    fn flush(&self) {}
}

/// This prints the given String out on the Tunneload instance on the error
/// log level
fn error(content: &str) {
    let ptr = content.as_ptr();
    let length = content.len();

    unsafe {
        raw::log_error(ptr as i32, length as i32);
    }
}
