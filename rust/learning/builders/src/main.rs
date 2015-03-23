use std::io::{BufReader, BufWriter};
use std::fs::{File, OpenOptions};

fn main() {
    println!("Hello, world!");

    let mut options = OpenOptions::new();
    options.read(true).write(true).append(true);

    /*
    let file = match options.open("test.txt") {
        Ok(file) => file,
        Err(..)  => panic!("the disco."),
    };

    let file2 = match options.open("test.txt") {
        Ok(file) => file,
        Err(..)  => panic!("at the disco."),
    };

    let writer: &BufWriter<File> = &BufWriter::new(file);
    let reader: &BufReader<File> = &BufReader::new(file2);
    */

    let reader = ReaderBuilder::new().path("test.txt").finalize();

}

struct Reader {
    path: String,
    reader: BufReader<File>,
    writer: BufWriter<File>,
}

struct ReaderBuilder {
    path: String,
}

impl ReaderBuilder {
    fn new() -> ReaderBuilder {
        ReaderBuilder { path: "".to_string() }
    }

    fn path(&mut self, path: &str) -> &mut ReaderBuilder {
        self.path = String::from_str(path);
        self
    }

    fn finalize(&self) -> Reader {
        let mut options = OpenOptions::new();
        options.read(true).write(true).append(true);

        let file = match options.open(&self.path) {
            Ok(file) => file,
            Err(..)  => panic!("at the disco."),
        };

        let file2 = match options.open(&self.path) {
            Ok(file) => file,
            Err(..)  => panic!("at the disco."),
        };

        //let buf_writer: &BufWriter<File> = &BufWriter::new(file);
        //let buf_reader: &BufReader<File> = &BufReader::new(file2);

        Reader {
            path: self.path.clone(),
            reader: BufReader::new(file),
            writer: BufWriter::new(file2),
        }

    }
}
