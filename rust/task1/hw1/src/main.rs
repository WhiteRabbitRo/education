use std::io;
use rand::random_range;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    
    let limit = (n as f64).sqrt() as u64;
    for i in 2..=limit {
        if n % i == 0 {
            return false
        }
    }
    true
}

fn roman_to_int_char(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0
    }
}

fn roman_to_int(s: &str) -> i32 {
    let mut sum = 0;
    let mut prev = 0; 
    let mut current;

    if s.is_empty() {
        return 0;
    }

    for c in s.chars() {
        current = roman_to_int_char(c);
        if current == 0 {
            return 0; // invalid roman number
        }
        if current > prev {
            sum -= prev * 2;
        }
        sum += current;
        prev = current;
    }
    sum
}

fn guessing_game() {
    let secret_number = random_range(1..=100);
    println!("Guess the number!");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }

    println!("Again? (y/n)");
    let mut again = String::new();
    io::stdin()
        .read_line(&mut again)
        .expect("Failed to read line");
    if again.trim() == "y" {
        guessing_game();
    }
}

fn min_max_sum(slice: &[i32]) -> (i32, i32, i32) {
    let mut min = slice[0];
    let mut max = slice[0];
    let mut sum = slice[0];
    for i in 1..slice.len() {
        let n = slice[i];
        if n < min {
            min = n;
        }
        if n > max {
            max = n;
        }
        sum += n;
    }
    (min, max, sum)
}

fn main() {
    println!("Input number: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u64 = input.trim().parse().expect("Invalid number");
    println!("Is prime: {}", is_prime(n));

    println!("Input roman number: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    let n: i32 = roman_to_int(&input);
    println!("Roman number: {}", n);

    guessing_game();

    let slice = [1, 2, 3, 4, 5];
    let (min, max, sum) = min_max_sum(&slice);
    println!("Min: {}, Max: {}, Sum: {}", min, max, sum);
}
