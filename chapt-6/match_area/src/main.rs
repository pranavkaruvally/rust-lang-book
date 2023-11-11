enum Shape {
    Rectangle(u32, u32),
    Circle(f64)
}

fn main() {
    let fig = Shape::Rectangle(5, 3);
    let _fig2 = Shape::Circle(5.0);
    let area: f64 = match fig {
        Shape::Rectangle(x, y) => (x*y) as f64,
        Shape::Circle(r) => 3.14*r*r
    };

    println!("Area: {area}");
}
