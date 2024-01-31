use rand::Rng;
use std::io::stdin;

fn main() {
    let version = "0.0.1";

    let mut try_counter = 0;

    println!("guessing Game - {}", version);

    // create random nummer
    let secret: u32 = rand::thread_rng().gen_range(0..=100);

    // todo remove this println
    // println!("The secret Number is: {secret}");

    loop {
        // update try_counter
        try_counter += 1;
        println!("Try: {try_counter}");

        // handle the input
        println!("Please input your guess(1-100):");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Can not read input");

        // parse string to Number
        // this can not handle invalid inputs well
        // let guess: u32 = guess.trim().parse().expect("Please type a Number");

        // this will start the for loop again if this will fail
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // write the input
        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You won!");
                println!("You need {try_counter} tries");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}
