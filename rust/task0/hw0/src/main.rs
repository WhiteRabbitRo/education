use std::io;

fn main() {
    
    let mut num1 = String::new();
    let mut num2 = String::new();

    let a : i32;
    let b : i32;

    loop {
        println!("Input first number:  ");
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        
        a = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    loop {
        println!("Input second number:  ");
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");

        b = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    loop {
        let mut operation = String::new();

        println!("Input operation:  ");
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        match operation.trim() {
            "+" => {
                println!("{} + {} = {}", a, b, a + b);
            },
            "-" => {
                println!("{} - {} = {}", a, b, a - b);
            },
            "*" => {
                println!("{} * {} = {}", a, b, a * b);
            },
            "/" => {
                match b {
                    0 => {
                        println!("Can't divide by 0");
                    },
                    _ => {
                        println!("{} / {} = {}", a, b, a / b);
                    },
                };
            },
            _ => continue
        };

        break;
    }
}
