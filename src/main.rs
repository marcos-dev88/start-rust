use std::io;
use rand::{Rng, thread_rng, prelude::ThreadRng};
use std::cmp::Ordering;

fn main() {
    println!("Guess the word!");
    
    
    loop {
        let mut rgn: ThreadRng = thread_rng();

        let secret_number: u32 = rgn.gen_range(0..100);
        
        let mut guess: String = String::new();
        
        println!("The secret number is {}", secret_number);

        
        println!("\nPlease input your guess.");

        io::stdin()
        .read_line(&mut guess)
        .expect("Invalid input");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        // .expect("please type a number");
        
        println!("You guessed -> {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }    
        }
    }
}
