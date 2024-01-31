use rand::Rng;
use std::io::stdin;

fn main() {
    let version = "0.0.1";

    let mut try_counter = 0;

    println!("guessing Game - {}", version);

    // create random nummer
    let secret: u32 = rand::thread_rng().gen_range(0..=100);

    loop {
        // update try_counter
        try_counter += 1;
        println!("Try: {try_counter}");

        // handle the input
        println!("Please input your guess(1-100):");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Can not read input");

        // this will start the for loop again, if parse to int failed for some reason
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // write the input
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
