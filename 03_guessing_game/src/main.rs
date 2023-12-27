// Import necessary libraries.
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // Print a welcome message to the user.
    println!("Guess the Number!");

    // Generate a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Understanding the start..=end Syntax
    //  - The start..=end syntax in Rust is known as an inclusive range. It includes both the start and end values in the range.
    //  - In the expression 1..=100:
    //      - 1 is the start of the range.
    //      - 100 is the end of the range.
    //  - The ..= operator creates a range that includes both 1 and 100.
    //  - This is different from start..end (without the equal sign), which is an exclusive range and does not include the end value.
    //  - For example, 1..100 would include numbers from 1 to 99.

    // Print the secret number for debugging purposes.
    // Note: Comment this out or remove in the final version for a real challenge!
    println!("The secret number is: {secret_number}");

    // Enter a loop to allow multiple guesses.
    loop {
        // Prompt the user to input their guess.
        println!("Please input your guess");

        // Create a variable to store the user's guess.
        let mut guess = String::new();

        // Read the user's input from the standard input (stdin).
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // Convert the guess from a string to a number (u16).
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,     // If parsing is successful, use the number.
            Err(_) => continue, // If parsing fails, skip the rest of the loop and ask for input again.
        };

        // Print the guessed number.
        println!("You guessed: {guess}");

        // Compare the guessed number to the secret number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // Guess is less than the secret number.
            Ordering::Equal => {
                println!("Guessed it correctly!"); // Guess is equal to the secret number.
                break; // Exit the loop.
            }
            Ordering::Greater => println!("Too large!"), // Guess is greater than the secret number.
        }
    }
}
