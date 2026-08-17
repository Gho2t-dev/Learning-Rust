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
2 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
  |              ++++++++++++

For more information about this error, try `rustc --explain E0284`.
error: could not compile `no_type_annotations` (bin "no_type_annotations") due to 1 previous error.

Integers:

Table 3-1: Integer Types in Rust

Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
Architecture-dependent	isize	usize

Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive, where n is the number of bits that variant uses. So, an i8 can store numbers from −(27) to 27 − 1, which equals −128 to 127. Unsigned variants can store numbers from 0 to 2n − 1, so a u8 can store numbers from 0 to 28 − 1, which equals 0 to 255.

