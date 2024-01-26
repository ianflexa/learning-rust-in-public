use std::fs::File;
use std::io::ErrorKind;

#[warn(dead_code)]
enum Result<T, E> {
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

}

// ***** notes *****
/*


*/
