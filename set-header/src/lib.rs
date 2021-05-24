use tunneload_plugin::{
    request, response, Headers, MiddlewareRequest, MiddlewareResponse, Response, StatusCode,
};

#[request]
fn handle_req<'a>(req: MiddlewareRequest) -> Result<(), Response<'a>> {
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

#[response]
fn handle_resp(resp: MiddlewareResponse) {
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
