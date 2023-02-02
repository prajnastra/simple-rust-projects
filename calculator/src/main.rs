use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut sign = String::new();

    println!("Enter number: ");
    io::stdin()
        .read_line(&mut num1)
        .expect("Not able to read input");

    println!("Enter sign: ");
    io::stdin()
        .read_line(&mut sign)
        .expect("Not able to read input");

    println!("Enter another number: ");
    io::stdin()
        .read_line(&mut num2)
        .expect("Not able to read input");

    let num1: f64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let num2: f64 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let sign = sign.trim();

    let result = match sign {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => 0.0,
    };

    println!("Result is {result}");
}
