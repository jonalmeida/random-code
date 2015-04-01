fn main() {
    /*
    let my_pointer = Data::<String> { stuff: &Some("This has a lifetime pointer".to_string()) };
    println!("my_pointer: {}", (*my_pointer.stuff).unwrap());
    */
    let my_pointer = Data::<&str> { stuff: &Some("This has a lifetime pointer") };
    println!("my_pointer: {}", (*my_pointer.stuff).unwrap());
}

struct Data<'a, T: 'a> {
    stuff: &'a Option<T>,
}
