extern mod extra;

use extra::sort::quick_sort;
use std::io::file_reader;
use std::path::Path;


fn main() {
    let reader = file_reader(&Path("names.txt")).unwrap();
    let contents = reader.read_c_str();
    let mut names: ~[&str] = contents.split_iter(',').transform(|x| x.slice(1, x.len() - 1)).collect();

    quick_sort(names, |a, b| a <= b);

    let mut counter: int = 0;
    let mut sum: int = 0;
    let first_letter: int = 'A' as int;

    foreach name in names.iter() {
        counter += 1;
        let name = name.to_owned();
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
