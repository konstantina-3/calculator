use std::io;

fn get_number() -> f32 {
    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    a.trim().parse().expect("Please type a number!")
}

fn main() {
    println!("Enter number:");
    let mut result = get_number();

    let mut order = true;
    let mut current_operator = '=';

    loop {
        match order {
            false => {
                println!("Enter number: ");
                let num = get_number();

                result = match current_operator {
                    '+' => result + num,
                    '-' => result - num,
                    '*' => result * num,
                    '/' => result / num,
                    _ => panic!("Invalid operator"),
                };

                println!("{}", result);

                order = true;
            }
            true => {
                println!("Enter operator (+, -, *, /, =):"); // '=' will finish calculation
                let mut op = String::new();

                io::stdin()
                    .read_line(&mut op)
                    .expect("Failed to read line");

                current_operator = op.chars().take(1).last().unwrap();

                if current_operator == '=' {
                    println!("{}", result);
                    break;
                }

                order = false;
            }
        }
    }
}
