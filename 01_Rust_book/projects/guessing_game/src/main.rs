use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // One line comment
    /*
        And the usual block comment,
        just like with Cadence
    */
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("\nPlease input your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if secret_number - guess < 4 {
                    println!("So close! But still too small...")
                }
                else if secret_number - guess < 10 {
                    println!("You're getting closer, but still under. Carry on!")
                }
                else {
                    println!("Way too small man!");
                }
            },
            Ordering::Greater => {
                if guess - secret_number < 4 {
                    println!("So close! But still too big...")
                }
                else if guess - secret_number < 10 {
                    println!("You're getting closer, but still over. Carry on!")
                }
                else {
                    println!("Way too big bub!")   
                }
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
