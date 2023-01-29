

fn main(){
    let get = HTTPMethod::GET;
    let delete = HTTPMethod::DELETE;
    let post = HTTPMethod::POST;
    let put = HTTPMethod::PUT;
    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}
struct Request {
    path: String,
    query_string: Option<String>,
    method: HTTPMethod,
}

enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}