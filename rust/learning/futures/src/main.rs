use std::sync::Future;
use std::task;
use std::rand;

fn main() {
    let mut delayed_value = Future::spawn(move || {
        12345i
    });

    println!("value = {}", delayed_value.get());

    let result = task::try(move || {
        if rand::random() {
            println!("OK");
        } else {
            panic!("oops!");
        }
    });
}
