extern "C" {
    /// Returns the Method of the Request as serialized
    pub fn get_method() -> i32;

    /// Returns the Length of the Path of the Request
    pub fn get_path_length() -> i32;

    /// Copies the Path of the Request into the WASM-Memory starting
    /// at the given target-Address
    pub fn get_path(target_addr: i32);

    /// Sets the Path on the Request with the given bytes
    pub fn set_path(path_addr: i32, path_length: i32);

    /// Sets the header on the Request with the bytes of the key
    /// being stored at the key_addr with the length key_length,
    /// and the bytes of the value being stored at the value_addr
    /// with the length value_length
    pub fn set_header_text(key_addr: i32, key_length: i32, value_addr: i32, value_length: i32);

    /// Checks if a Header for the given Key is available
    ///
    /// Returns:
    /// * 0: False
    /// * != 0: True
    pub fn has_header(key_addr: i32, key_length: i32) -> i32;

    /// Returns the Length of the Header-Value
    ///
    /// # Returns:
    /// * `value <= 0`: The Header was not set on the Request
    /// * `value > 0`: The Header was set and it represents the Size of the Value
    pub fn get_header_length(key_addr: i32, key_length: i32) -> i32;

    /// Copies the Value for given Header-Key into the target-addr
    pub fn get_header(target_addr: i32, key_addr: i32, key_length: i32);
}
