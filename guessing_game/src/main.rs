use std::io; //Importing input/ouput standard library
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Input your guess: ");

        let mut guess = String::new(); //Creating a variable ; new is an associated
                                       //of String type, returns empty string

        io::stdin()
            .read_line(&mut guess) //& means we're passing it as a reference, read_line
                                   //takes what user types in stdin and appends it
                                   //to string w/o overwriting the contents.
                                   //read_line returns a Result value which is an enum
            .expect("Failed to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //We're converting guess
        //to  a number to
        //compare same types
        //
        //parse() returns a Result type which is an enum with variants as Ok and Err

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

//In Rust, variables are immutable by default
//We add mut keyword to make a variable mutable

//Rust provides crates (collection of Rust source code files)
//This project is a binary crate, rand is a library crate
