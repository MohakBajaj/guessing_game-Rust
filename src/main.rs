use std::io;

fn main() {
    println!("Hello, world! This is a Guessing Game!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    println!("Thanks for playing!");
}
