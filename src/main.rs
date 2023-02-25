use std::io;

fn get_number() -> f32 {
    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    a.trim().parse().expect("Please type a number!")
}

fn main() {
    println!("Enter the first number:");
    let a = get_number();

    println!("Enter the second number:");
    let b = get_number();

    println!("Enter the operation symbol (+, -, * or /):");
    let mut op = String::new();

    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read line");

    let op = op.trim();

    let result:f32  = match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => panic!("Invalid operator"),
    };

    println!("{} {} {} = {}", a, op, b, result);

}
