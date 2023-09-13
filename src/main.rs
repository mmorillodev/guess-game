use guessing::Guess;
use std::cmp::Ordering::*;
use std::io;

mod guessing;

fn main() {
    let mut guess = Guess::new();
    println!("Guess the number!");

    loop {
        println!("Please input your guess. [1..=100]");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    continue;
                }
                num
            }
            Err(_) => continue,
        };

        guess.set_value(input);

        println!("You guessed: {}", guess.value());

        match guess.compare_guess() {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
