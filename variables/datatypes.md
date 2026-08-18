Data Types
Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so that it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.

Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, such as when we converted a String to a numeric type using parse in the “Comparing the Guess to the Secret Number” section in Chapter 2, we must add a type annotation, like this:

'''rust
let guess: u32 = "42".parse().expect("Not a number!");
'''

If we don’t add the : u32 type annotation shown in the preceding code, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use:

$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `guess` an explicit type
  |
2 |     let guess: /*Type*/ = "42".parse().expect("Not a number!");
  |              ++++++++++++

For more information about this error, try `rustc --explain E0284`.
error: could not compile `no_type_annotations` (bin "no_type_annotations") due to 1 previous error.

Integers:

Table 3-1: Integer Types in Rust

Length Signed Unsigned
8-bit i8 u8
16-bit i16 u16
32-bit i32 u32
64-bit i64 u64
128-bit i128 u128
Architecture-dependent isize usize

Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive, where n is the number of bits that variant uses. So, an i8 can store numbers from −(27) to 27 − 1, which equals −128 to 127. Unsigned variants can store numbers from 0 to 2n − 1, so a u8 can store numbers from 0 to 28 − 1, which equals 0 to 255.

FOR GENERAL USE i32 is the default

if you "overflow" your numbers for debug it results in panic but for release it wraps around and starts at 0 again
For example relaseing let x: u8 = 256 will result in x being 0 for release and not compile with panic at debug compile time

Floating point numbers:
there are 2 types: f32 and f64. of which f64 is the default as its about as fast as f32 on modern cpus and way more acurate.


Basic Mathematical operations

fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

Char types are seen in the following example:
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
Note that we specify char literals with single quotation marks, as opposed to string literals, which use double quotation marks.

Compound types:
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

Tuples:
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
type annotations are optional in this example

two variants to access data inside a tuple:
Nr1 mapping the values
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

Nr2 . annotation
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

Arrays:
Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

fn main() {
    let a = [1, 2, 3, 4, 5];
}

If you are unsure if you should use an array or a vector you should use a vector.
arrays live on the stack and vectors on the heap? more on that later

Easy Rule to Remember

Array = Many of the same thing

let temperatures = [22, 24, 21, 23];

Tuple = A group of related things

let employee = ("Fabian", 1234, true);

A good analogy:

Array = Egg carton containing only eggs 🥚🥚🥚🥚
Tuple = A lunch box containing a sandwich, an apple, and a drink 🥪🍎🥤

You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

let a: [i32; 5] = [1, 2, 3, 4, 5];
Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

let a = [3; 5];

array elemt access

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing, like this:

fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}