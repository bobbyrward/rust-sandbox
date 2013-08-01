extern mod std;

use std::vec;
use std::vec::raw;
use std::cell::Cell;
use std::rt::io::{Reader, Writer, Listener};
use std::rt::io::net::tcp::{TcpListener, TcpStream};
use std::rt::io::net::ip::{IpAddr};


pub fn allocate_buffer(buffer_size: uint) -> ~[u8] {
    let mut buffer: ~[u8] = std::vec::with_capacity(buffer_size);
    unsafe { std::vec::raw::set_len(&mut buffer, buffer_size); }
    return buffer
}


pub fn serve_forever(bind_address: IpAddr, handle_request: &fn(stream: TcpStream)) {
    let mut listener = match TcpListener::bind(bind_address) {
        Some(x) => x,
        None    => {
            fail!("Unable to bind to " + bind_address.to_str());
        }
    };

    loop {
        let stream = Cell::new(listener.accept().unwrap());

        do spawn {
            handle_request(stream.take());
        }
    }
}
