//! This module contains a couple of logging related parts that can be used
//! to output logs in the Tunneload instance

use crate::raw;

/// This prints the given String out on the Tunneload instance on the error
/// log level
pub fn error(content: &str) {
    let ptr = content.as_ptr();
    let length = content.len();

    unsafe {
        raw::log_error(ptr as i32, length as i32);
    }
}
