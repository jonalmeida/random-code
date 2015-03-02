
fn main() {
    for x in range(1i, 100i) {
        if x % 5 == 0 && x % 3 == 0 {
            println!("FizzBuzz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", x);
        }
        //println!( "{}", x );
    }
}
