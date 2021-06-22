use stream_httparse::Method;

use crate::raw::{self, REQUEST_RESSOURCE_ID};

/// A thin "Wrapper" to make it easier to interact with
/// the current Request from Tunneload.
///
/// This will perform all the needed raw calls to Tunneload
/// and provides a safe API for the user
pub struct MiddlewareRequest {
    path_length: usize,
    body_size: usize,
    max_header_length: usize,
}

impl MiddlewareRequest {
    /// Creates a new Request instance with the given Parameters for better
    /// Performance
    pub fn new(path_length: i32, body_size: i32, max_header_length: i32) -> Self {
        Self {
            path_length: path_length as usize,
            body_size: body_size as usize,
            max_header_length: max_header_length as usize,
        }
    }

    /// Loads the Method from Tunneload
    pub fn get_method(&self) -> Option<Method> {
        let code = unsafe { raw::action_get_method() };
        Method::wasm_deserialize(code)
    }

    /// Updates the Headers of the Request by adding/overwriting
    /// a Header with the given Key-Value pair
    pub fn set_header(&self, key: &str, value: &str) {
        unsafe {
            raw::action_set_header(
                REQUEST_RESSOURCE_ID,
                key.as_ptr() as i32,
                key.len() as i32,
                value.as_ptr() as i32,
                value.len() as i32,
            );
        }
    }

    /// Attempts to load the Path of the Request from Tunneload
    pub fn get_path(&self) -> String {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.path_length);

        let addr = buffer.as_ptr() as i32;
        unsafe {
            raw::action_get_path(addr);
            buffer.set_len(self.path_length);
        }

        String::from_utf8(buffer).unwrap()
    }

    /// Sets the Path of the Request to the new given Path
    pub fn set_path(&self, n_path: &str) {
        unsafe {
            raw::action_set_path(n_path.as_ptr() as i32, n_path.len() as i32);
        }
    }

    /// Checks if a Header with the given Key is present on the Request
    pub fn has_header(&self, key: &str) -> bool {
        let value = unsafe {
            raw::action_has_header(REQUEST_RESSOURCE_ID, key.as_ptr() as i32, key.len() as i32)
        };

        value != 0
    }

    /// Attempts to load the Value of the Header with the matching
    /// Key from Tunneload
    pub fn get_header(&self, key: &str) -> Option<String> {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.max_header_length);
        unsafe {
            let actual_length = raw::action_get_header(
                REQUEST_RESSOURCE_ID,
                buffer.as_ptr() as i32,
                key.as_ptr() as i32,
                key.len() as i32,
            );
            if actual_length <= 0 {
                return None;
            }
            buffer.set_len(actual_length as usize);
        }

        let result = String::from_utf8(buffer).unwrap();
        Some(result)
    }

    /// Loads the Body of the Request
    pub fn get_body(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.body_size);

        unsafe {
            raw::action_get_body(REQUEST_RESSOURCE_ID, buffer.as_ptr() as i32);
            buffer.set_len(self.body_size);
        }

        buffer
    }

    /// Sets the Body on the Request to the given Data
    pub fn set_body(&self, data: &[u8]) {
        unsafe {
            raw::action_set_body(
                REQUEST_RESSOURCE_ID,
                data.as_ptr() as i32,
                data.len() as i32,
            );
        }
    }
}
