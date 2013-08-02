use std::rt::io::net::ip::Ipv4;
use std::rt::io::net::tcp::TcpStream;
use std::rt::io::{Reader, Writer, Listener};
use http::server::HTTPServer;
use http::parser::{Parser, ParserCallbacks};
use std::str;
use std::vec;
use std::hashmap::HashMap;
use std::ptr::to_unsafe_ptr;



#[path = "http"]
mod http {
    mod server;
    mod parser;
    mod http_parser;
}


pub fn allocate_buffer(buffer_size: uint) -> ~[u8] {
    let mut buffer: ~[u8] = vec::with_capacity(buffer_size);
    unsafe { vec::raw::set_len(&mut buffer, buffer_size); }
    return buffer
}


struct Request {
    url: ~str,
    headers: ~HashMap<~str, ~str>,
    body: ~str,
    callbacks: Option<ParserCallbacks>,
    current_header: ~str,
    parse_finished: bool,
}

impl Request {
    pub fn new() -> Request {
        let request = Request {
            url: "".to_owned(),
            headers: ~HashMap::new::<~str, ~str>(),
            body: "".to_owned(),
            callbacks: None,
            parse_finished: false,
            current_header: "".to_owned(),
        };

        request
    }

    pub fn parse(&mut self, parser: &mut Parser, data: &[u8]) -> bool{
        let unsafe_self = to_unsafe_ptr(&self);

        let callbacks = ParserCallbacks {
            on_message_begin:       || unsafe { (*unsafe_self).on_message_begin() },
            on_url:                 |data| unsafe { (*unsafe_self).on_url(data) },
            on_header_field:        |data| unsafe { (*unsafe_self).on_header_field(data) },
            on_header_value:        |data| unsafe { (*unsafe_self).on_header_value(data) },
            on_headers_complete:    || unsafe { (*unsafe_self).on_headers_complete() },
            on_body:                |data| unsafe { (*unsafe_self).on_body(data) },
            on_message_complete:    || unsafe { (*unsafe_self).on_message_complete() },
        };

        self.callbacks = Some(callbacks);

        let callbacks = &self.callbacks.unwrap();
        let parsed_bytes = parser.execute(data, callbacks);

        if parsed_bytes != data.len() {
            fail!(fmt!("parser failure: %? %? %?", parsed_bytes, data.len(), parser.status_code()));
        }

        self.callbacks = None;

        true
    }

    pub fn on_message_begin(&self) -> bool {
        true
    }
    pub fn on_url(&mut self, data: ~[u8]) -> bool {
        let data_str = str::from_bytes(data);
        self.url = data_str;

        true
    }
    pub fn on_header_field(&mut self, data: ~[u8]) -> bool {
        let data_str = str::from_bytes(data);
        self.current_header = data_str;

        true
    }
    pub fn on_header_value(&mut self, data: ~[u8]) -> bool {
        let data_str = str::from_bytes(data);
        self.headers.insert(self.current_header.clone(), data_str);

        true
    }
    pub fn on_headers_complete(&self) -> bool {
        true
    }
    pub fn on_body(&mut self, data: ~[u8]) -> bool {
        let data_str = str::from_bytes(data);
        self.body = data_str;

        true
    }
    pub fn on_message_complete(&mut self) -> bool {
        self.parse_finished = true;
        true
    }
}


fn handle_request(mut tcp_stream: TcpStream) {
    let mut parser = Parser();
    let mut request = Request::new();
    let mut dest_buffer = allocate_buffer(512);

    let peer = tcp_stream.peer_name().unwrap().to_str();
    println(fmt!("Accepting connection from: %s", peer));

    loop {
        match tcp_stream.read(dest_buffer) {
            Some(x) => {
                request.parse(&mut parser, dest_buffer.slice_to(x));
            },
            None => {
                break
            },
        };

        let write = |str_out: ~str| {
            let bytes = str_out.to_bytes_with_null();
            tcp_stream.write(bytes.slice(0, bytes.len() - 1));
        };

        if request.parse_finished {
            write(~"HTTP/1.1 200 OK\r\n");
            write(~"Content-Length: 0\r\n");
            write(~"Content-Type: text/plain\r\n");
            write(~"\r\n");
            break;
        } else {
            break;
        }
    }

    println(fmt!("%?", request));
    println(fmt!("Closing connection with: %s", peer));
}


fn main() {
    let mut server = HTTPServer::new(Ipv4(127, 0, 0, 1, 8000), handle_request);
    server.serve_forever();
}
