use tunneload_plugin::{request, MiddlewareRequest};

#[request]
fn handle_req(req: MiddlewareRequest) -> Result<(), ()> {
    let path = req.get_path();

    let prefix = "/test";

    let n_path = path.strip_prefix(prefix).unwrap();
    req.set_path(n_path);

    Ok(())
}
