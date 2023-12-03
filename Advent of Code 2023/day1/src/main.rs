use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("foo.txt").expect("Failed to open file.");
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).expect("Failed to read file");
}
