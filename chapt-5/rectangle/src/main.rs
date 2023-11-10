struct Rectangle {
    length: u32,
    breadth: u32
}

//We add the same area functionality using method

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }
}

fn main() {
    let r = Rectangle {
        length: 12,
        breadth: 10
    };
    let a = area(&r);
    println!("Area is {}", a);
    println!("Area using method is {}", r.area());
}

fn area(r: &Rectangle) -> u32 {
    r.length * r.breadth
}