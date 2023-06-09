use rand::Rng;
use std::io;
fn main() {
    println!("Welcome to guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please provide an input");

    let mut guess = String::new();
    println!("Your input will be checked against a random number");

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
}
