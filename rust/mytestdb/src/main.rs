extern crate txtdb;

use txtdb::{controller, factory};

fn main() {
    let mut reader = controller::Reader::new(&Path::new("database.txt"));
    //let factory: factory::Factory = factory::RecordFactory::new(reader);
    //let database = txtdb::Txtdb::new(factory);

    reader.insert_str("record_added");
    println!("Hello, world!");
}
