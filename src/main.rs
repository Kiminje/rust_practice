use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    println!("The secret number is: {}", secret_number);
    
    println!("Please input your guess.");
    let mut guess = String::new(); // creates new, empty string


    //Receiving User Input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //potential failure

    println!("You guessed: {}", guess);

    let x = 5; 
    let y = 10;
    println!("x = {} and y = {}", x, y);
}