use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let rand_num = rand::thread_rng().gen_range(1..=100);

    println!("Welcome to The Number Guessing Game");

    println!("Please enter a guess between 1 - 100: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 5,
    };

    match guess.cmp(&rand_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
