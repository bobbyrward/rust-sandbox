extern mod std;

use std::cell::Cell;
use std::rt::io::Listener;
use std::rt::io::net::tcp::{TcpListener, TcpStream};
use std::rt::io::net::ip::{IpAddr};

pub type RequestHandler = extern fn(tcp_stream: TcpStream);

pub struct HTTPServer {
    bind_address: IpAddr,
    request_handler: RequestHandler
}

impl HTTPServer {
    pub fn new(bind_address: IpAddr, request_handler: RequestHandler) -> HTTPServer {
        HTTPServer { bind_address: bind_address, request_handler: request_handler }
    }

    pub fn serve_forever(&mut self) -> HTTPServer {
        let mut listener = match TcpListener::bind(self.bind_address) {
            Some(x) => x,
            None    => {
                fail!("Unable to bind to " + self.bind_address.to_str());
            }
        };

        loop {
            let stream = Cell::new(listener.accept().unwrap());
            let handler = self.request_handler;

            do spawn {



                handler(stream.take());
            }
        }
    }

}
