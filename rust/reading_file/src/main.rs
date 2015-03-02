use std::io::BufferedReader;
use std::io::{File, Open, Read};

fn main() {
    let path = Path::new("tests/base-test.txt");
    let file = File::open_mode(&path, Open, Read);
    let mut reader = BufferedReader::new(file);
    //println!("Hello, world!");
    let mystring = reader.read_line().ok().expect("Error 1");
    print!("{}", mystring);
    let mystring2 = reader.read_line().ok().expect("Error 2");
    print!("{}", mystring2);
    let mystring3 = reader.read_line().ok().expect("Error 3");
    print!("{}", mystring3);
}
