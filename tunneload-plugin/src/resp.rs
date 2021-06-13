use stream_httparse::StatusCode;

use crate::raw;

/// A thin "Wrapper" to make it easier to interact with the
/// current Response from Tunneload
///
/// This will perform all the needed raw calls to Tunneload
/// and provides a safe API for the user
pub struct MiddlewareResponse {
    body_size: usize,
    max_header_length: usize,
}

impl MiddlewareResponse {
    /// Creates a new Response instance
    pub fn new(body_size: i32, max_header_length: i32) -> Self {
        Self {
            body_size: body_size as usize,
            max_header_length: max_header_length as usize,
        }
    }

    /// Loads the StatusCode of the Response from Tunneload
    pub fn get_status_code(&self) -> StatusCode {
        let raw_code = unsafe { raw::action_get_status_code() };

        StatusCode::wasm_deserialize(raw_code).unwrap()
    }

    /// Updates the Headers of the Request by adding/overwriting
    /// a Header with the given Key-Value pair
    pub fn set_header(&self, key: &str, value: &str) {
        unsafe {
            raw::action_set_header(
                key.as_ptr() as i32,
                key.len() as i32,
                value.as_ptr() as i32,
                value.len() as i32,
            );
        }
    }

    /// Attempts to load the Value of the Header with the matching
    /// Key from Tunneload
    pub fn get_header(&self, key: &str) -> Option<String> {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.max_header_length);
        unsafe {
            let actual_length = raw::action_get_header(
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
            raw::action_get_body(buffer.as_ptr() as i32);
            buffer.set_len(self.body_size);
        }

        buffer
    }

    /// Sets the Body on the Response to the given Data
    pub fn set_body(&self, data: &[u8]) {
        unsafe {
            raw::action_set_body(data.as_ptr() as i32, data.len() as i32);
        }
    }
}
