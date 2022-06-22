use std::{io, cmp::Ordering};
use rand::{thread_rng, Rng};
use colored::*;

fn main() {

    loop {
        println!("Please input your guess : ");

        let mut guess = String::new();
        
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        if guess.trim().to_lowercase() == "q" {
            println!("{}", "Thank you for Playing !".on_bright_yellow());
            break;
        }
    
        let mut rng = thread_rng();
        let secret_number: u32 = rng.gen_range(0..10);
    
        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}","Please enter no between 1 - 10".on_bright_magenta());
                continue;
            },
        };
    
        println!("You guessed : {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small !".red()),
            Ordering::Greater => println!("{}","Too big !".red()),
            Ordering::Equal => {
                println!("{}","You win !".green());
                break;
            },
        };
    }
}
