/////////////////////////////////////////////////////////
use rand::Rng;
use std::cmp::Ordering;
/// Guessing Game
/////////////////////////////////////////////////////////
///
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //[1, 100]
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    //store values with varaibles
    let mut guess = String::new();

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //transform input string into a number
        let number: Result<i32, _> = guess.trim().parse();
        match number {
            Ok(n) => {
                println!("You guessed: {}", n);
                match n.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                };
                println!("calculating: {guess} x 2 = {}", n * 2);
            }
            Err(_) => println!("Please type a number!"),
        }
    }
}
