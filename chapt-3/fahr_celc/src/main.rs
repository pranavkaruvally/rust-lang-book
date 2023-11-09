use std::io;

fn main() {
    let mut fahr = String::new();
    println!("Enter temperature in fahrenheit:");
    io::stdin()
        .read_line(&mut fahr)
        .expect("Read error!");

    let fahr: f64 = fahr.trim().parse()
        .expect("Enter a number!");

    let celc = fahr_to_celcius(fahr);
    println!("{fahr} degree fahr is {celc} degree celcius");
}

fn fahr_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}