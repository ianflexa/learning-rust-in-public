fn main() {
    // variables and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 600000;
    println!("The value of x is: {x}");
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    //error[E0384]: cannot assign twice to immutable variable `x`

    // constants
    const TWO_MINUTES_IN_SECONDS: u32 = 60 * 2; // all uppercase with underscores between words

    println!("2 minutes in seconds: {}", TWO_MINUTES_IN_SECONDS);

    // shadowing
    let x = 5; // shadowed 

    let x = x + 1 - 2; // by this variable

    {
        let x = x * 20;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // let spaces = "   ";
    // let spaces = spaces.len();
    //     ^^^^^^ help: if this is intentional, prefix it with an underscore: `_spaces`


    // let mut spaces = "   ";
    // //  ^^^^ help: remove this `mut`
    // let spaces = spaces.len();


    // Data Types

    // let guess = "42".parse().expect("Not a number!");

    //  let guess: /* Type */ = "42".parse().expect("Not a number!");
    //           ++++++++++++

    let guess: u32 = "42".parse().expect("Not a number!"); // : u32 type annotation
    println!("{}", guess);

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation

    // Compound Types

    let tup: (i32, f64, u8) = (100, 100.0, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let one = tup.2;
    println!("one: {}", one);


    // Array Type

    // array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array,
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("first index: {}", a[0]);

    let a = [3; 5]; // same as:  let a = [3, 3, 3, 3, 3]

    // Functions 

    another_function();


    //              ***** notes *****
    /*
        - Rust compiler guarantees that when you state that a value won’t change
        - not possible to use `mut` with constants
        - char literals with single quotes, as opposed to string literals
        - char type is four bytes in size
        - for function and variable names we use snake_case
        - Rust doesn’t care where you define your functions
    */
}

fn another_function() {
    println!("Another fn!");
}
