struct Point {
    x: int,
    y: int,
}

enum OptionalInt {
    Value(int),
    String(String),
    Empty,
}

fn main() {
    let x:int;
    x = 5;
    let y:int = if x == 5i { 10i } else { 15i };
    println!("Value of y: {}", y);

    let x = add_numbers(5, 6);
    println!("add_numbers: {}", x);

    let (a, b): (int, int) = (7, 8);
    println!("Giving inverse_typle two values: {} {}", a, b);
    let (retuple_first, retuple_second) = inverse_tuple(a, b);
    println!("This is the retuple: {} {}", retuple_first, retuple_second);

    let a_new_point: Point = Point { x: 0, y: 4 };
    println!("My new point has co-ordinates x: {}, and y: {}", a_new_point.x, a_new_point.y);

    println!("Comparing x: {} and y: {}, we know that x is {} y.", x, y, compare(x, y));

    let some_x: int = 5;
    match some_x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("n/a"),
    }

    let some_y: Ordering = Less;
    match some_y {
        Less => println!("Less than!"),
        Equal => println!("Equal than!"),
        Greater => println!("Greater than!"),
    }

    let some_x: OptionalInt = OptionalInt::Value(6);
    let some_y: OptionalInt = OptionalInt::String("Foooo".to_string());
    let some_z: OptionalInt = OptionalInt::Empty;

    match some_x {
        OptionalInt::Value(n) => println!("Value(int) was selected with the int value {}", n),
        OptionalInt::String(strng) => println!("String(String) was selected with the String value {}", strng),
        OptionalInt::Empty => println!("Empty!"),
    }

    match some_y {
        OptionalInt::Value(n) => println!("Value(int) was selected with the int value {}", n),
        OptionalInt::String(strng) => println!("String(String) was selected with the String value {}", strng),
        OptionalInt::Empty => println!("Empty!"),
    }

    match some_z {
        OptionalInt::Value(n) => println!("Value(int) was selected with the int value {}", n),
        OptionalInt::String(strng) => println!("String(String) was selected with the String value {}", strng),
        OptionalInt::Empty => println!("Empty!"),
    }


    for x in range(0i, 10i) {
        print!( "{} ", x );
    }
    print!("\n");

    let mut counter = 1i;
    loop {
        print!( "{} ", counter );
        if counter % 10 == 0 { break; }
        counter += 1;
    }
    print!("\n");

    for x in range(0i, 10i) {
        if x % 2 == 0 { continue; }

        print!( "{} ", x );
    }
    print!("\n");

    let mut s = "Hello".to_string();
    s.push_str(", world.");

    println!("{}", compare_array(&[1i, 2i, 3i]));

    let mut v = vec![5i, 6i, 9i, 10i];
    println!("Vector size: {}", do_something_with_vectors(&v));
    v.push(2i);
    println!("Vector size: {}", do_something_with_vectors(&v));

    let input = std::io::stdin().read_line().ok().expect("Failed!");
    print!("{}", input);
}

/// This is a function that adds numbers together
/// # Arguments
///
/// * `x` - The first int
/// * `y` - The second int
///
/// ```rust
/// add_numbers(5, 6);
/// ```
fn add_numbers(x: int, y: int) -> int {
    x + y
}

/// Returns the tuple provided in inverse order
/// # Arguments
///
/// * `first` - An integer value.
/// * `second` - An integer value.
/// # Example
///
/// ```
/// let (x, y) = inverse_tuple(7, 8);
/// ```
fn inverse_tuple(first: int, second: int) -> (int, int) {
    (second, first)
}

/// Compare two interger values and returns an appropriate
/// Ordering value depending on the compare.
/// # Arguments
///
/// * `first` - An integer value.
/// * `second` - An integer value.
/// # Example
///
/// ```
/// let order: Ordering = compare( 3i, 4i );
/// println!("The order is: {}", order);
/// ```
fn compare(first: int, second: int) -> Ordering {
    if first < second {
        Ordering::Less
    } else if first > second {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// This has nothing to do with comparing.
/// # Arguments
///
/// * `array` - An int array.
/// # Example
///
/// ```
/// println!("{}", compare_array(&[1i, 2i, 3i]));
/// ```
fn compare_array(array: &[int]) -> uint {
    println!("Printing compare_array() now:");
    for element in array.iter() {
        print!("{} ", element);
    }
    print!("\n");
    array.len()
}

/// Does some magic with vectors.
/// # Arguments
///
/// * `vector` - An int array passed with the memory address.
/// # Example
///
/// ```
/// println!("{}", do_something_with_vectors(vec![1i, 2i, 3i]));
/// ```
fn do_something_with_vectors(vector: &Vec<int>) -> uint {
    vector.len()
}
