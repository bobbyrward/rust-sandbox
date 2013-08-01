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
        match listener.accept() {
            Some(stream) => {
                let mut buf = [1, ..512];
                let mut stream = stream;

                let read_count = stream.read(buf);

                match read_count {
                    Some(x) => stream.write(buf.slice_to(x)),
                    None => (),
                };
            },
            None         => break,
        };
    }

}
