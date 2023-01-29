
use server::Server;
use http::request::Request;
fn main(){
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {
    pub mod request {
        use super::httpmethod::HTTPMethod;
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: HTTPMethod,
        }
    }

    mod httpmethod {
        pub enum HTTPMethod {
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
    }
   
}
