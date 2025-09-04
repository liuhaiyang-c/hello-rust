use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let welcome = String::from("Guess the number!");
    println!("{}", welcome);
    println!("Please input your guess");

    let the_num = get_random_num();

    loop {
        let guess = get_user_input();

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a Number!");
                continue;
            }
        };

        match guess.cmp(&the_num) {
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
            Ordering::Greater => println!("your guess is greater"),
            Ordering::Less => println!("your guess is smaller"),
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    return match std::io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => String::new(),
    };
}

fn get_random_num() -> i32 {
    rand::rng().random_range(1..=100) // 1..100 左闭右开
}
