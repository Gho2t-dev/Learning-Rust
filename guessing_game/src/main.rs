use std::{cmp::Ordering, io}; //The io library comes from the standard library, known as std
use rand::Rng; //

fn main() { // main function that runs as soon as the programm is executed
    println!("Guess the number!"); // print line macro 

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        //print the secret number (only for debuging :) )
        //println!("The secret number is: {secret_number}");

        println!("Please input your guess.");

        let mut guess = String::new(); // creates a mutable variable called "guess"
        // Example:
        // let apples = 5; is immutable
        // let mut bananas = 5; is mutable

        // String::new() :: syntax in the ::new line indicates that new is an associated function of the String type
        // this just assigns an mutable empty string to "guess"

        io::stdin() // call the stdin function from the io module, which will allow to handle user input
            .read_line(&mut guess) // calls the readline methon from stdin reading what the user typed and appending it to the guess variable
            // does NOT overwrite but only appends
            // & is a reference that lets you access data and is immutable so mut is neccesary
            .expect("Failed to read line");
        // Result’s variants are Ok and Err
        // expect crashes the programm and displays a message. later there should be error handling instead

        println!("You guessed: {guess}");

        // converts guess to u32, trims whitespace(eg \n) parses(converts to u32) and OLD (expect(to catch errors))
        let guess: u32 = match guess.trim().parse(){ // Another match here handling result that parse gives us enum 
            //  OK by passing num and err with just continuing the loop
            Ok(num) => num,
            Err(_) => continue, //The underscore, _, is a catch-all value handling ALL kinds of errors
        };

        // .cmp() compares and returns Ordering enum with states less greater and equal
        match guess.cmp(&secret_number) {   // match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Correct!!!");
                break
            }
        }
    }
}

//Summary from the book
//Summary
//This project was a hands-on way to introduce you to many new Rust concepts: let, match, functions, the use of external crates, and more. 
//In the next few chapters, you’ll learn about these concepts in more detail. 
//Chapter 3 covers concepts that most programming languages have, such as variables, data types, and functions, and shows how to use them in Rust. 
//Chapter 4 explores ownership, a feature that makes Rust different from other languages. Chapter 5 discusses structs and method syntax, and Chapter 6 explains how enums work.