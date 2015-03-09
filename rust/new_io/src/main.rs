/*
use std::io::{BufWriter, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;

fn main() {
    let path = Path::new("test_file.txt");
    //File::create(&path.clone());
    let mut options = OpenOptions::new();
    options.write(true).append(true);
    let file = match options.open(&path.clone()) {
        Ok(file) => file,
        Err(e) => panic!("Unable to open a write buffer to {} path.\n{}", path.display(), e),
    };
    let mut writer = BufWriter::new(&file);
    writer.write_all(b"test\n");
//    writer.flush();   // I guess we don't need flush with write_all
    println!("We've reached the end, a line should in the {} file.", path.display());
}
*/


use std::io::{BufReader, BufRead, Read, Error};
use std::fs::{File, OpenOptions};
use std::path::Path;

fn main() {
    let path = Path::new("test_file.txt");
    let path2 = Path::new("test_file2.txt");

    // or we can also just use a string slice:
    let path2 = "test_file2.txt";

    // We create file options to write
    let mut options = OpenOptions::new();
    options.write(true);

    // Both of these should be valid
    let file: Result<File, Error> = options.open(path);
    let file2: Result<File, Error> = options.open(path2);
}
