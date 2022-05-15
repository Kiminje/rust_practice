// use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    loop {
        println!("Guess the number!");
        let secret_number = rand::thread_rng().gen_range(1..101);
        
        println!("The secret number is: {}", secret_number);
        
        println!("Please input your guess.");
        let mut guess = String::new(); // creates new, empty string
        
        //Receiving User Input
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //potential failure
        

        //trim: eliminate whitespace at the beginning and the end (elminate \n or \r\n)
        //parse: only work on characters that can logically be converted into numbers. 

        // let guess: u32 = guess.trim().parse().expect("please type a number");
        let guess: u32 = match guess.trim().parse()   {
            Ok(v) => v,
            Err(_) => {
                println!("You type {}please type a number", guess);
                continue;
            }
        };

        


        println!("You guessed: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less  => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Match!");
                break;
            }
        }
    }
    
    let x = 5; 
    let y = 10;
    println!("x = {} and y = {}", x, y);
}