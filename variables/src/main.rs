// Variables in rust

//fn main() {
//    let x = 5;
//    println!("The value of x is: {x}");
//    x = 6;
//    println!("The value of x is: {x}");
//}
//This example shows how the compiler helps you find errors in your programs. 
//Compiler errors can be frustrating, but really they only mean your program isn’t safely doing what you want it to do yet; 
//they do not mean that you’re not a good programmer! Experienced Rust programmers still get compiler errors.

// variables can be made mutable by adding "mut" infront of the variable name:
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
//This will compile fine and print 5 and then 6

//Constants in Rust
//Constants CANNOT be made mutable and HAVE to be anntoted with the coresponding type
//Constants can be declared everywhere so they are usefull for global scope
//Example of a constant:
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//Constants all upercase
//Usefull for things such as the maximum number of points any player of a game is allowed to earn, or the speed of light.

//Shadowing

//fn main() {
//    let x = 5;
//
//    let x = x + 1;
//
//    {
//        let x = x * 2;
//        println!("The value of x in the inner scope is: {x}");
//    }
//
//    println!("The value of x is: {x}");
//}

//Shadowing temporarily reassigns the value of a variable and is automatically reset once the current scope ends.
//in this example the first print will give out 12
//and the second one will go back to 6 as x is 5 and then gets shadowed to be 6 in the main scope
//so basically shadowing a variable ends when the scope ends.

//Mutating also does not let you change the variable type but shadowing does bc you are creating a new variable
//for ex:
//  let spaces = "   ";
//  let spaces = spaces.len();
//this will be possible only with shadowing if done with a mutable variable like this:
//  let mut spaces = "   ";
//  spaces = spaces.len();
//the code will not compile with error saying that we’re not allowed to mutate a variable’s type