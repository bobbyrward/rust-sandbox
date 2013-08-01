use std::rt::io::net::ip::Ipv4;
use std::rt::io::net::tcp;
use http::server::{serve_forever, allocate_buffer};

#[path = "http"]
mod http {
    mod server;
    mod parser;
    mod http_parser;
}


fn handle_request(steam: std::rt::io::net::tcp::TcpStream) {
    let mut dest_buffer = allocate_buffer(512);
    let peer = stream.peer_name().unwrap().to_str();
    println(fmt!("Accepting connection from: %s", peer));

    loop {
        match stream.read(dest_buffer) {
            Some(x) => stream.write(dest_buffer.slice_to(x)),
            None => {
                break
            },
        };
    }

    println(fmt!("Closing connection with: %s", peer));
}


fn main() {
    let listen_address = Ipv4(127, 0, 0, 1, 8000);

    serve_forever(listen_address, handle_request);
}
