#![warn(missing_docs)]
//! This Crate provides all the "Framework" stuff for creating a Plugin
//! for the Tunneload load-balancer/ingress

/// This contains all the Raw Methods exposed by the Host, Tunneload.
pub mod raw;

pub use stream_httparse::{Header, Headers, Response, StatusCode};
pub use tunneload_plugin_macros::{parse_config, request, response};

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

/// This Trait needs to be implemented for any Configuration type that
/// you want to use in the Plugin
pub trait Config: Sized {
    /// This function serializes the Configuration into a block of memory
    ///
    /// # Returns
    /// * A Ptr to the beginning of the block of memory
    /// * The Size of the block of memory
    fn serialize_data(&self) -> (*const u8, usize);

    /// This function attempts to deserialize the given Block of memory back
    /// into the Configuration for the Plugin
    ///
    /// # Params:
    /// * A Ptr to the beginning of the block of memory
    /// * The Size of the block of memory
    fn deserialize_data(addr: *mut u8, size: usize) -> Option<Self>;
}
