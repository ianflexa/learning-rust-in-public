fn main() {
    // scope
    {
        // `s` is not valid here
        let s = "hey"; // `s` is valid from the point forward

        // do stuff with `s`
    } // end of the scope, `s` is no longer valid here

    // the String type

    let mut s = String::from("hey");

    s.push_str("you!"); // push_str() appends a literal to a String

    println!("{}", s);

    {
        // init of the scope
        let mut s = String::from("yo"); // here memory is requested from the memory allocator at runtime.

        // do stuff
        s.push_str("!");

        println!("{}", s);
    } // scope is over. Rust calls `drop` automatically at the closing curly bracket.
      // memory is automatically returned once the variable that owns it goes out of scope.

    // Variables and Data Interacting with Move

    let x = 5;
    let y = x; // two 5 values are pushed onto the stack.

    let s1 = String::from("hello");
    let s2 = s1;

    /*
       |     s1      |       -------------------------------       |     s2      |
       | name | value|       `     | index | value |        |      | name | value|
       | ptr  |  ----|-------->    |   0   |   h   |        |------| ptr  |------|
       | len  |  5   |             |   1   |   e   |               | len  |  5   |
       | cpct |  5   |             |   2   |   l   |               | cpct |  5   |
                                   |   3   |   l   |
                                   |   4   |   o   |
                                   *data on the heap*

       len: how much memory, in bytes, the contents of the String are currently using
       cpct: total amount of memory, in bytes, that the String has received from the allocator.


       s1 and s2 pointer to the same data on the heap

    */

    {
        let s1 = String::from("hello");
        let s2 = s1; // here Rust considers s1 as no longer valid to avoid  double free error

        // println!("{}, world!", s1);
        //    ^^ value borrowed here after move
    }

    // Variables and Data Interacting with Clone

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deeply copy the heap data of the String

    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: Copy
    let x = 5;
    let y = x; // no difference between deep and shallow copying here, because i32 is on the stack and we know the size at compile time.
               // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are

    println!("x = {}, y = {}", x, y);

    let s = String::from("hey"); // `s` comes into the scope

    take_ownership(s); // `s` value moves to the function

    // `s` is no longer valid  here.

    let x: i32 = 10; // `x` comes into the scope

    makes_copy(x); // `x` is i32, i32 is Copy

    // `x` still into the scope

    // Return Values and Scope

    {
        let s1 = gives_ownership(); // gives_ownershio() moves its return value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which moves its return value into s3
    }// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.
}

// Ownership and Functions

fn take_ownership(some_string: String) {
    // `some_string` comes into the scope
    println!("{}", some_string);
} // `some_string` goes out of the scope, `drop` is called, memory is free again

fn makes_copy(some_int: i32) {
    // `some_int` comes into the scope
    println!("{}", some_int);
} // `some_int` goes out of the scope and nothing special happens.

// Return Values and Scope

fn gives_ownership() -> String {
    let some_string = String::from("hey hye"); // `some_string` comes into the scope
    some_string // `some_string` is returned and moves out to the  calling function
}

fn takes_and_gives_back(some_string: String) -> String {
    // `some_string` comes into the scope
    some_string // `some_string` is returned and moves out to the  calling function
}

// ***** notes *****
/*
    - Each value in Rust has an owner;
    - There can only be one owner at time
    - When the owner goes out of scope, the value will be dropped
    - String type is stored on the heap instead of the stack
    - String type can be mutated and literals can't be.
    - string literals are fast bc we now the size at compile time (bc they're immutable)
    - When a variable goes out of scope, Rust calls a special function called `drop`
    - A String is made up of three parts, shown on the left:
        1. Pointer (ptr) to the memory that holds the contents of the string
        2. Length (len)
        3. Capacity  (cpct)
    - Rust has a feature for using a value without transferring ownership, called `references`.

*/
