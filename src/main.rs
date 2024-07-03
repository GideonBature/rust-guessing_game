use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let rand_num = generate_random();

    println!("Welcome to The Number Guessing Game\n");

    loop {
        
        println!("Please enter a guess between 1 - 100: ");

        let mut guess = String::new();

        get_user_input(&mut guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn generate_random() -> u32 {
    let rand_num = rand::thread_rng().gen_range(1..=100);
    rand_num
}

fn get_user_input(guess: &mut String) -> &mut String {
    io::stdin()
        .read_line(guess)
        .expect("Failed to read line");
    guess
}
