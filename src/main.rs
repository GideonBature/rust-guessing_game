use std::io;

fn main() {
    println!("Welcome to The Number Guessing Game");

    println!("Please enter a guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("{}", guess);
}
