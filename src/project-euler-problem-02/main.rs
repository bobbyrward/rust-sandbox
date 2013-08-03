use std::int;
use std::comm::stream;
use std::comm::Port;
use std::comm::Chan;


type MaybeInt = Option<int>;


fn fib_generator(max: int, chan: &Chan<MaybeInt>) {
    let mut a = 0;
    let mut b = 1;

    loop {
        let next = a + b;

        if next > max {
            break;
        }

        b = a;
        a = next;

        chan.send(Some(next));
    }

    chan.send(None);
}


fn main() {
    let (port, chan): (Port<MaybeInt>, Chan<MaybeInt>) = stream();

    do spawn {
        fib_generator(4000000, &chan);
    }

    let mut accum: int = 0;

    loop {
        let next: MaybeInt = port.recv();

        match next {
            Some(x) if x % 2 == 0 => accum += x,
            Some(_) => loop,
            None => break,
        };
    }

    println(fmt!("%d", accum));
}

