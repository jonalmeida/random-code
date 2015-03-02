use std::io::{BufWriter, Write};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

fn main() {
    let path = PathBuf::new("test_file.txt");
    //File::create(&path.clone());
    let mut options = OpenOptions::new();
    options.write(true).append(true);
    let file = match options.open(&path.clone()) {
        Ok(file) => file,
        Err(e) => panic!("Unable to open a write buffer to {} path.\n{}", path.display(), e),
    };
    let mut writer = BufWriter::new(&file);
    writer.write_all(b"test\n");
    writer.flush();
    println!("We've reached the end, a line should in the {} file.", path.display());
}
