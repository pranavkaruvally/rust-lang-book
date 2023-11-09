use std::io;

fn main() {
    println!("Enter n:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Read Error!");
    let n = n.trim().parse().expect("Enter a number!");
    let f = fib(n);
    println!("The nth fibonacci number is {f}");
}

fn fib(n: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    
    for _i in 1..n {
        second = first + second;
        first = second - first;
    }

    return second;
} 