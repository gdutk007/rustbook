use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    let random_number = rand::thread_rng().gen_range(1..=30);
    //println!("The secret number is {random_number}.");
    
    
    loop{
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line!");

        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };//.expect("please enter a number!");
        
        println!("You guessed: {guess}");

        match guess.cmp(&random_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}