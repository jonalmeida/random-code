
struct Circle {
    radius: f64,
    x: f64,
    y: f64,
}

impl Circle {
    fn area(&self) -> f64 {
       std::f64::consts::PI * (self.radius * self.radius)
    }
}

