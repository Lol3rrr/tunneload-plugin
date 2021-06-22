use tunneload_plugin::{
    acceptor::{self, Sender},
    logging, Acceptor, AcceptorPlugin, Headers, Method,
};

async fn sending() {
    let mut tx = Sender::new();

    loop {
        let _message = tx.send().await.unwrap();

        tokio::task::yield_now().await;
    }
}

async fn main_loop(max_cons: i32) {
    let mut con_id = 0;

    let mut headers = Headers::new();
    headers.append("Host", "lol3r.net");
    headers.append("Content-Length", 0);
    let req = tunneload_plugin::Request::new("HTTP/1.1", Method::GET, "/", headers, &[]);
    let (head, _body) = req.serialize();

    loop {
        if con_id > max_cons {
            tokio::task::yield_now().await;
            continue;
        }

        let receiver = acceptor::new_connection(con_id);
        receiver.recv(&head);
        // Temporarily disabled because it would cause a EOF to be send as
        // it is 0-length
        // receiver.recv(body);

        con_id += 1;

        tokio::task::yield_now().await;
    }
}

struct LoadTestingHandler;

#[AcceptorPlugin]
impl Acceptor for LoadTestingHandler {
    fn run() {
        logging::init(log::Level::Info).unwrap();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();

        let mut handles = Vec::new();

        handles.push(runtime.spawn(sending()));
        handles.push(runtime.spawn(main_loop(50)));

        runtime.block_on(futures::future::join_all(handles));
    }
}
