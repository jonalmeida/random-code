use std::sync::RwLock;
use std::io::{File};

fn main() {
    let file = File::create(&Path::new("tests/base-test.txt")).ok().expect("Unable to create test file");
    let lock: RwLock<File> = RwLock::new(file);
    println!("Hello, world!");
    let mut w1 = lock.write().unwrap();
    w1.write_str("10 11\n20 21\n");
}
