
fn main() {
    hello::print_hello();
    let i      = 1i;
    let j      = &i;
    println!("This is i:        {}", i);
    println!("This is &i via j: {}", j);

    let x = OptionalInt::Value(5i);
    let y = OptionalInt::Missing;

    match x {
        OptionalInt::Value(..)  => println!("Got me an int here!"),
        _                       => println!("Something I don't care about"),
    }

    match y {
        OptionalInt::Value(..)  => println!("Got me an int here!"),
        _                       => println!("Something I don't care about"),
    }

    let mut z = &5i;

    match z {
         ref mut r    => println!("Got a value: {}", r),
    }

    let a: Point = Point { x: 0i, y: 5i };

    match a {
        Point { x: u, y: v }    => println!("({}, {})", u, v),
    }

    //let v = vec!["match", "2", "something"];
    let v = vec!["match", "2"];

    match v.as_slice() { [a, val]   => println!("Found a '{}' after a '{}'!", val, a),
       e @ _            => println!("Other stuff I don't care about! {}", e),
    }

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("Radius of the circle: {}", c.area());

    let d = Circle::new(0.0, 0.0, 4.0);
    println!("Radius of the 'new' cirlce: {}, {}, {}", d.radius, d.x, d.y);

    fn handler(x: int) -> int {
        println!("We got our {}, but here's a 4 instead", x);
        4i
    }

    hello::closure(5, handler)
}

mod hello {
    pub fn print_hello() { println!("Hello, world!"); }

    pub fn closure( x: int, y: |int| -> int) {
        if x == 5 {
            y(3);
            3i;
        }
    }
}

enum OptionalInt {
    Value(int),
    Missing,
}

struct Point {
    x: int,
    y: int,
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn new(x: f64, y:f64, radius:f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}
