use std::io::TempDir;
use std::io::File;
use std::io::fs;
use std::io;

fn main() {
    let tmpdir = match TempDir::new("txtdb-tests") {
        Ok(dir) => dir,
        Err(e)  => panic!("Couldn't create temp dir")
    };
    println!("Path: {}", tmpdir.path().as_str().unwrap());
    //fs::mkdir(&tmpdir.path().clone(), io::USER_FILE);
    //fs::mkdir(&Path::new("/tmp/txtdb-tests"), io::USER_FILE);

    let final_path = tmpdir.path().join("foo.txt");

    println!("Hello, world: {}", tmpdir.path().display());
    File::create(&final_path);
    //File::create(&Path::new("/tmp/foobar"));
    loop {}
}
