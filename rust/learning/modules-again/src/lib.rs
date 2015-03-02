use std::io::BufferedReader;
use std::io::File;

pub mod reader;

fn read_file() {
    let fname = "test.txt";
    let path = Path::new(fname);
    let mut file = BufferedReader::new(File::open(&path));
    for line_iter in file.lines() {
        println!("This our line: {}", line_iter.unwrap());
    }
    //let mut file = File::create(&Path::new("message.txt"));
    //file.write(b"hello, file!\n");
}

#[test]
fn test_read_file() {
    read_file();
}
