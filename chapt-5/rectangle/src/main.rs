struct Rectangle {
    length: u32,
    breadth: u32
}

fn main() {
    let r = Rectangle {
        length: 12,
        breadth: 10
    };
    let a = area(r);
    println!("Area is {}", a);
}

fn area(r: Rectangle) -> u32 {
    r.length * r.breadth
}