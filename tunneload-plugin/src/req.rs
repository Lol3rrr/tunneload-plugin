use stream_httparse::Method;

use crate::raw;

pub struct MiddlewareRequest {}

impl MiddlewareRequest {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_method(&self) -> Option<Method> {
        let code = unsafe { raw::get_method() };
        Method::wasm_deserialize(code)
    }

    pub fn set_header(&self, key: &str, value: &str) {
        unsafe {
            raw::set_header_text(
                key.as_ptr() as i32,
                key.len() as i32,
                value.as_ptr() as i32,
                value.len() as i32,
            );
        }
    }

    pub fn get_path(&self) -> String {
        let size = unsafe { raw::get_path_length() } as usize;

        let mut buffer: Vec<u8> = Vec::with_capacity(size);

        let addr = buffer.as_ptr() as i32;
        unsafe {
            raw::get_path(addr);
            buffer.set_len(size);
        }

        String::from_utf8(buffer).unwrap()
    }

    pub fn set_path(&self, n_path: &str) {
        unsafe {
            raw::set_path(n_path.as_ptr() as i32, n_path.len() as i32);
        }
    }

    pub fn has_header(&self, key: &str) -> bool {
        let value = unsafe { raw::has_header(key.as_ptr() as i32, key.len() as i32) };

        value != 0
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        let length = unsafe { raw::get_header_length(key.as_ptr() as i32, key.len() as i32) };
        if length <= 0 {
            return None;
        }

        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        unsafe {
            raw::get_header(
                buffer.as_ptr() as i32,
                key.as_ptr() as i32,
                key.len() as i32,
            );
            buffer.set_len(length as usize);
        }

        let result = String::from_utf8(buffer).unwrap();
        Some(result)
    }
}
