use std::io; // pythonic.... from std import io
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin() //calling the stdin function from the standin library
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {}", guess);

}