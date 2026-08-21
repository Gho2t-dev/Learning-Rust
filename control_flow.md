This file will go into more detail about control flow in rust.

To be exact, if expressions and loops.

If expressions:
as in many other language, if statemets let you execute code based on a condition. if a certain condition is met the code associated
with said condition will execute, if not then the code will be skipped.

for if statements in rust, refer to the "branches" crate in this repository.

fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}


Loops:
Rust can be used to execute a specific partof code over and over again.
this is done with loops:

fn main() {
    loop {
        println!("again!");
    }
}

