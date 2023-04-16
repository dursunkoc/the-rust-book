use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let holy_grail = rand::thread_rng().gen_range(1..101);

    println!("The target is {}", holy_grail);

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read the line!");

        let guess = match guess.trim().parse::<i32>(){
            Ok(value) => value,
            Err(_) =>{
                println!("Invalid guess!");
                continue;
            }
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&holy_grail) {
            Ordering::Greater => println!("Lower your guess!"),
            Ordering::Less => println!("Increase your guess!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        };
    }
}
