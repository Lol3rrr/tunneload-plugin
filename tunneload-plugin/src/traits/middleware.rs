use stream_httparse::Response;

use crate::{Config, MiddlewareRequest, MiddlewareResponse};

/// This Trait defines the interface for a Middleware-Plugin
///
/// # Note:
/// The Middleware-Plugin only exports the Parts that you implement
/// from this Trait.
/// So if you only implement the Request-Handler, the Plugin will only
/// be called into for Requests, and vice-versa.
pub trait Middleware: Config {
    /// This Handler deals with all incoming Requests
    fn handle_request<'resp>(&self, _req: MiddlewareRequest) -> Result<(), Response<'resp>> {
        Ok(())
    }

    /// This Handler deals with all the incoming Requests
    fn handle_response(&self, _req: &MiddlewareRequest, _resp: MiddlewareResponse) {}
}
