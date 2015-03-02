fn modify(x: &mut usize) -> usize { *x + 1 }

fn main() {
    println!("Hello, world!");
    let mut x = 5us;
    let mut y = &mut x;
    /*if x == 5 {
        let y = &mut x;
        *y = 1;
        println!("{}", y);
        return;
    }*/
    x = modify(y);
    println!("{}", x);
}
