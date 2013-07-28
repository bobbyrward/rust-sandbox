extern mod extra;

use std::path::Path;
use std::str;


fn main() {
    let reader = std::io::file_reader(&Path("names.txt")).unwrap();
    let contents = reader.read_whole_stream();

    let split_contents: ~[&[u8]] = contents.split_iter(|x| *x == ',' as u8).collect();
    let mut names: ~[~str] = split_contents.map(|x| str::from_bytes(x.slice(1, x.len() - 1)));

    extra::sort::quick_sort(names, |a, b| *a <= *b);

    let mut counter: int = 0;
    let mut sum: int = 0;
    let first_letter: int = 'A' as int;

    for names.iter().advance |name| {
        counter += 1;
        let name = name.clone();
        let bytes: ~[u8] = name.to_bytes_with_null();
        let score: int = bytes.iter().fold(0, |a: int, b: &u8| {
            match *b {
                0 => a,
                _ => a + *b as int - first_letter + 1,

            }
        });

        sum += counter * score;
    }

    println(fmt!("%?", sum));
}
