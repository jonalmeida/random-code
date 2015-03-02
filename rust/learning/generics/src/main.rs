fn main() {
    println!("Hello, world!");
    let x: Option<int> = Some(5i);
    let y = match x {
        Some(num)   => num,
        None        => { println!("You got screwed!"); return; }
    };
    println!("This our unboxed value: {}", y);

    let z: Option<f64> = Some(1.929202f64);
    println!("We got a 64 bit float here: {}",
        match z {
            Some(value) => value,
            None        => { return; },
        }
    );

    match inverse(25.0f64) {
        Result::Ok(value)     => { println!("We got a good value: {}", value) },
        Result::Err(val)      => { println!("{}", val) }
    }
}

fn inverse<T>(x: T) -> Result<T, String> {
    if x == 0.0 { return Result::Err("x cannot be zero!".to_string()); }

    Result::Ok(1.0 / x)
}
