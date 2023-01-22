use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number: {secret_num}");

    loop {
        println!("Please input your guess.");

        // rust varilables are immutable by default.
        // So, we add `mut` to make it mutable.
        let mut guess = String::new();

        // passing guess by reference here.
        // references are immutable by default.
        // We therefore make it mutable with `&mut`.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only numbers!");
                continue;
            }
        };

        println!("Your Guess: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
