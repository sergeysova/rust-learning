
use std::fs::File;

fn file_create() {
    let mut file = try!(File::create("example.txt"));
    try!(file.write_all(b"Hello, world!"));
}

fn file_exists() {
}

fn file_read() {
}

fn main() {
    file_create();
}
