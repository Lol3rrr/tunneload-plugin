use tunneload_plugin::{parse_config, request, Config, MiddlewareRequest, Response};

#[request(PrefixConfig)]
fn handle_req<'a>(conf: PrefixConfig, req: MiddlewareRequest) -> Result<(), Response<'a>> {
    let path = req.get_path();
    let req_path_len = path.len();
    let prefix: &str = conf.content.as_ref();
    let prefix_len = prefix.len();

    if prefix_len > req_path_len {
        return Ok(());
    }
    if &path[0..prefix_len] != prefix {
        return Ok(());
    }

    req.set_path(&path[prefix_len..].to_owned());

    Ok(())
}

struct PrefixConfig {
    content: String,
}

impl Config for PrefixConfig {
    fn serialize_data(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }
    fn deserialize_data(data: &[u8]) -> Option<Self> {
        let content = String::from_utf8(data.to_vec()).unwrap();

        Some(Self { content })
    }
    fn len(&self) -> usize {
        String::len(&self.content)
    }
}

#[parse_config]
fn parse_config_init(data: String) -> PrefixConfig {
    PrefixConfig { content: data }
}
