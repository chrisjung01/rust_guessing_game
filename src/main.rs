use rand::Rng;
use std::io::{stdin, stdout, Write};

fn main() {
    let version = "0.0.4";

    let mut try_counter = 0;

    println!("guessing Game - {}", version);

    let secret: u32 = rand::thread_rng().gen_range(0..=100);

    loop {
        try_counter += 1;
        println!("Try: {try_counter}");

        println!("Please input your guess(1-100):");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Can not read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You won! Need {try_counter} tries");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}
