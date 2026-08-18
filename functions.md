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

returning a value:
Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->).

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}