use std::cell::Cell;
use std::rt::io::{Reader, Writer, Listener};
use std::rt::io::net::tcp::*;
use std::rt::io::net::ip::*;


fn allocate_buffer(buffer_size: uint) -> ~[u8] {
    let mut buffer: ~[u8] = std::vec::with_capacity(buffer_size);
    unsafe { std::vec::raw::set_len(&mut buffer, buffer_size); }
    return buffer
}


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

            let mut dest_buffer = allocate_buffer(512);
            let mut stream = stream.take();

            loop {
                match stream.read(dest_buffer) {
                    Some(x) => stream.write(dest_buffer.slice_to(x)),
                    None => {
                        break
                    },
                };
            }
        }
    }

}
