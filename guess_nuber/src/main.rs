use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop{
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("faild to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {  
            Ordering::Less=>{println!("too small")},
            Ordering::Equal=>{
                println!("you win");
                break;
            },
            Ordering::Greater=>{println!("too large")},
        }
    }
    
}
