use std::io; //The io library comes from the standard library, known as std

fn main() { // main function that runs as soon as the programm is executed
    println!("Guess the number!"); // print line macro 

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
}

// TODO rand num generation and game logic