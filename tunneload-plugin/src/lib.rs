#![warn(missing_docs)]
//! This Crate provides all the "Framework" stuff for creating a Plugin
//! for the Tunneload load-balancer/ingress

/// This contains all the Raw Methods exposed by the Host, Tunneload.
pub mod raw;

pub use stream_httparse::{Header, Headers, Response, StatusCode};
pub use tunneload_plugin_macros::{parse_config, Config, MiddlewarePlugin};

mod req;
pub use req::*;

mod resp;
pub use resp::*;

/// Loads the Configuration String from Tunneload
pub fn load_config_str(raw_size: i32) -> String {
    let size = raw_size as usize;

    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    unsafe {
        raw::get_config_str(buffer.as_ptr() as i32);
        buffer.set_len(size);
    }

    String::from_utf8(buffer).unwrap()
}

pub mod logging;

mod traits;
pub use traits::{Config, Middleware};
