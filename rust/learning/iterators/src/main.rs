fn main() {
    println!("Hello, world!");
    let nums = vec![1i, 2i, 3i];
    for num in nums.iter() {
        println!("{}", *num);
    }
    //let one_to_one_hundred = range(1i, 101i).collect::<Vec<int>>();
    //println!("We print one_to_one_hundred: {}", one_to_one_hundred);

    let greater_than_forty_two = range(1i, 100i)
                                    .find(|x| *x > 42i);

    match greater_than_forty_two {
        Some(_) => println!("Got some:"),
        None    => println!("Got none.:( "),
    }

    let sum = range(1i, 6i)
                .fold(0i, |sum, x| {
                        println!("accumulator: {}, element: {}", sum, x);
                        sum + x
                    }
                );
    println!("We're printing the sum from a fold: {}", sum);

    for i in std::iter::count(1i, 5i).take(3) {
        println!("{}", i);
    }

    for z in range(1i, 10i).filter(|&x| x % 2 == 0) {
        println!("Even: {}", z)
    }
}
