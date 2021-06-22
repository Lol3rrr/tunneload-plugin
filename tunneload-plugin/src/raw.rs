/// The Ressource-ID assosicated with Requests
pub const REQUEST_RESSOURCE_ID: i32 = 0;
/// The Ressource-ID assosicated with Responses
pub const RESPONSE_RESSOURCE_ID: i32 = 1;

extern "C" {
    /// This function instructs Tunneload to print the String, that
    /// is stored at the given PTR and with the given Length, to be
    /// printed with error log level
    pub fn log_error(buffer_ptr: i32, buffer_site: i32);

    /// Copies the Config-String into the WASM-Memory starting
    /// at the given target_addr
    pub fn get_config_str(target_addr: i32);

    /// Copies the previously parsed Configuration for the Module
    /// into the WASM-Memory starting at the given target
    /// addres
    pub fn get_config(target_addr: i32);

    /// Returns the Method of the Request as serialized
    pub fn action_get_method() -> i32;

    /// Returns the StatusCode of the Response as serialized
    pub fn action_get_status_code() -> i32;

    /// Copies the Path of the Request into the WASM-Memory starting
    /// at the given target-Address
    pub fn action_get_path(target_addr: i32);

    /// Sets the Path on the Request with the given bytes
    pub fn action_set_path(path_addr: i32, path_length: i32);

    /// Sets the header on the Request/Response with the bytes of the key
    /// being stored at the key_addr with the length key_length,
    /// and the bytes of the value being stored at the value_addr
    /// with the length value_length
    pub fn action_set_header(
        ressource: i32,
        key_addr: i32,
        key_length: i32,
        value_addr: i32,
        value_length: i32,
    );

    /// Checks if a Header for the given Key is available
    ///
    /// Returns:
    /// * 0: False
    /// * != 0: True
    pub fn action_has_header(ressource: i32, key_addr: i32, key_length: i32) -> i32;

    /// Copies the Value for given Header-Key into the target-addr.
    /// This also returns the actual amount of bytes that were
    /// written to the buffer
    ///
    /// # Returns:
    /// * `value <= 0`: The Header was not set on the Request
    /// * `value > 0`: The Header was set and it represents the Size of the Value
    pub fn action_get_header(
        ressource: i32,
        target_addr: i32,
        key_addr: i32,
        key_length: i32,
    ) -> i32;

    /// Copies the Body of the Request/Response into the buffer
    /// that starts at the given Address
    pub fn action_get_body(ressource: i32, target_addr: i32);

    /// Sets the Body on the Request/Response
    pub fn action_set_body(ressource: i32, addr: i32, length: i32);

    /// Informs Tunneload about a new Connection with the given ID
    pub fn acceptor_new_con(id: i32);

    /// Checks if Tunneload has any Data that needs to be send by this
    /// Acceptor-Plugin
    pub fn acceptor_has_send() -> i32;
    /// Attempts to load the Data that needs to be send from Tunneload into
    /// the Plugins-Memory starting at the given `target`-Address
    pub fn acceptor_send(target: i32) -> i32;

    /// Informs Tunneload about new Data for a given Connection and
    /// where it can find the Data to load it and use in the rest of the
    /// Load-Balancer
    pub fn acceptor_recv(id: i32, target: i32, size: i32);
}
