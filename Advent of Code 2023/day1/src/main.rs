use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let f = File::open("foo.txt").except("Failed to open file.");
    let mut buffer = String::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;
}
