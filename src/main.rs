use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number");

    let random_number = rand::thread_rng().gen_range(1..=10);

    println!("Random number is {random_number}");

    loop {
        println!("Please enter your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = guess.trim().parse().expect("Please enter a number");
    
        println!("You guessed: {guess}");
        
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

 
}