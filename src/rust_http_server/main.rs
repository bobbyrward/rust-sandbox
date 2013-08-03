use std::rt::io::net::ip::Ipv4;

use http::request::Request;
use http::response::Response;
use http::server::HTTPServer;


#[path = "http"]
mod http {
    mod http_parser;
    mod parser;
    mod request;
    mod response;
    mod server;
}


fn handle_request(request: Request) -> Response {
    let mut response = Response::new();

    response.set_status(200);
    response.set_header("Content-Type", "text/plain");
    response.set_body_str("body text");

    response
}


fn main() {
    let mut server = HTTPServer::new(Ipv4(127, 0, 0, 1, 8000), handle_request);
    server.serve_forever();
}
