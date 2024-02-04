use std::fs::File;
use std::io::{self, Read, ErrorKind};

#[warn(dead_code)]
enum ResultEg<T, E> {
    // generic types, T = value that will be returned in a success, E = error that will be returned in a failure
    Ok(T),  // Ok variant
    Err(E), // Err variant
}

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error), 
        /*
            thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }'

        */
    };

    // Matching on Different Errors

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() { // if not found, we create the file
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e), // handle if failed to create
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

    // Alternatives to using `match` with `Result<T, E>`
    let hey_file = File::open("hey.txt").unwrap_or_else(|error| { // much cleaner to read
        if error.kind() == ErrorKind::NotFound {
            File::create("hey.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error)
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // shortcuts for Panic on Error: `unwrap` and `expect`

    // let greeting_file = File::open("hello.txt").unwrap();
    /*
        thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
        code: 2, kind: NotFound, message: "No such file or directory" }'

     */

    let greeting_file = File::open("hello.txt").expect("hello.txt should exist");
    /*
        thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:16:23
    
     */



}

    // Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
        
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_two() -> Result<String, io::Error> {
    let mut username_file = File::open("hey.txt")?; // he ? placed after a Result value is defined to work in almost the same way as the match expressions 
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // useful when a function returns one error type to represent all the ways a function might fail
    Ok(username)
}


// ***** notes *****
/*
    - most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed. 
    - propagating the error: When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do.

*/
