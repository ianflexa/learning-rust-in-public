use rand::Rng; // Rng trait
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100); //start..=end
        println!("Please input your guess.");

        let mut guess = String::new(); // new instance of a String. `::new` is an associated function of the String type

        io::stdin() //stdin from io module to handle user's input
            .read_line(&mut guess) // get the user's input and store it in the guess variable. returns a `Result` value.
            .expect("Failed to read line"); // handle failures

        let guess: u32 = match guess.trim().parse() { //  shadow the previous value of guess with a new one.
            Ok(num) => num,
            Err(_) => continue, // Handle invalid input
        };

        println!("You guessed: {guess}");

        println!("The secret number is: {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    /*
        let apples = 5; immutable
            apples = 3;
    Error: "^^^^^^^^^^ cannot assign twice to immutable variable"

        let mut bananas = 5; mutable
        bananas = 7;  // This code compiles
     */
}

// *** notes ***

/*
    - std: standard library
    - io: input/output library (comes from std library)
    - `use` it's like `import` from js/ts
    - prelude is a set items defined in the std lib
    - we need to bring a type that we want to use into the scope if that type isn't in the prelude
    - variables are immutable by default, to make it mutable just add `mut` keyword
    - An associated function is a fn that's implemented a type
    - `&mut guess`  indicates that this argument is a reference, which means that we can access it without copying it into memory multiple times.
    - Result is an enum with variants Ok, which holds the successfully generated value, and Err, which holds information about the failure.
    - `{}` is placeholder.
    - Our project is a `binary crate`, which is an executable
    - `rand crate` is a library crate, which contains code to be used by other programs
    - default int:  i32

*/
