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


    //              ***** notes *****
    /*
        - Rust compiler guarantees that when you state that a value won’t change
        - not possible to use `mut` with constants
    */
}
