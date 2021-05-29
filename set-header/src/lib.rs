use tunneload_plugin::{
    Config, Headers, Middleware, MiddlewarePlugin, MiddlewareRequest, MiddlewareResponse, Response,
    StatusCode,
};

#[derive(Debug, Config)]
pub struct TestConf;

#[MiddlewarePlugin]
impl Middleware for TestConf {
    fn handle_request<'resp>(&self, req: MiddlewareRequest) -> Result<(), Response<'resp>> {
        let value = match req.get_header("test-key") {
            Some(v) => v,
            None => return Ok(()),
        };

        if value == "specific-value" {
            req.set_header("result-key", "result-true");
        } else {
            return Err(Response::new(
                "HTTP/1.1",
                StatusCode::InternalServerError,
                Headers::new(),
                Vec::new(),
            ));
        }

        Ok(())
    }

    fn handle_response(&self, resp: MiddlewareResponse) {
        let value = match resp.get_header("test-key") {
            Some(v) => v,
            None => return,
        };

        if value == "specific-value" {
            resp.set_header("result-key", "result-true");
        } else {
            resp.set_header("result-key", "result-false");
        }
    }
}
