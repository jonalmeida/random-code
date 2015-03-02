use std::rand;

fn main() {
    let x = rand::random::<usize>();
    let x_str = x.to_string();
    println!("Random str: {}", x);
}
