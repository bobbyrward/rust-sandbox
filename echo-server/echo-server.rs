use std::cell::Cell;
use std::rt::io::{Reader, Writer, Listener};
use std::rt::io::net::tcp::*;
use std::rt::io::net::ip::*;


fn main() {
    let listen_address = Ipv4(127, 0, 0, 1, 8000);

    let mut listener = match TcpListener::bind(listen_address) {
        Some(x) => x,
        None    => {
            fail!("Unable to bind to " + listen_address.to_str());
        }
    };

    loop {
        let stream = Cell::new(listener.accept().unwrap());
        do spawn {
            let mut buf = [1, ..512];
            let mut stream = stream.take();
            match stream.read(buf) {
                Some(x) => stream.write(buf.slice_to(x)),
                None => (),
            };
        }
    }

}
