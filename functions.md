These are my notes about functions in rust

you use "fn" to declare a new function, followed by the name of the function

simple and easy example function:

fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

functions can have parameters also called arguments like this:
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

PARAMETERS MUST BE TYPEANNOTATED.

