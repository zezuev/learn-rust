// The io library comes from the standard library
use std::io;
// The Rng trait defines methods that random number generators implement
use rand::Rng;
// An enum with the variants Less, Greater and Equal
use std::cmp::Ordering;

// The main function is the entry point into every Rust program
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        // Create a mutable string variable to store user input
        let mut guess = String::new();

        println!("Please input your guess.");

        // stdin returns an instance of Stdin, a handle to the standard input for the terminal
        io::stdin()
            .read_line(&mut guess) // The input is appended to the string
            .expect("Failed to read line!"); // read_line returns a Result value, which is an enum

        // we move from crashing on an error to handling the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catch-all value
        };

        // A match expression is made up of arms. An arm consists of a pattern to match against
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
