extern mod std;

use std::vec;
use std::cell::Cell;
use std::rt::io::{Reader, Writer, Listener};
use std::rt::io::net::tcp::TcpListener;
use std::rt::io::net::ip::{IpAddr};

use http::parser::Parser;
use http::request::Request;
use http::response::Response;


fn allocate_buffer(buffer_size: uint) -> ~[u8] {
    let mut buffer: ~[u8] = vec::with_capacity(buffer_size);
    unsafe { vec::raw::set_len(&mut buffer, buffer_size); }
    return buffer
}


pub type RequestHandler = extern fn(request: Request) -> Response;


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
                let mut parser = Parser();
                let mut request = Request::new();
                let mut dest_buffer = allocate_buffer(512);
                let mut tcp_stream = stream.take();

                loop {
                    match tcp_stream.read(dest_buffer) {
                        Some(x) => {
                            request.parse(&mut parser, dest_buffer.slice_to(x));
                        },
                        None => {
                            break
                        },
                    };

                    if request.parse_finished {
                        let mut response = handler(request);
                        tcp_stream.write(response.to_bytes());
                        break;
                    }
                }
            }
        }
    }
}
