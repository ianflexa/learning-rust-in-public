fn main() {
    println!("Hello, world!");

    another_another_function();

    another_fn_with_param(10);

    print_labeled_measurement(3, 'm');

    statement_and_expressions();

    let ten = return_ten();

    println!("number: {ten}");

    let two = plus_one(1);

    println!("number: {two}");
}

fn another_another_function() {
    println!("another fn");
}

fn another_fn_with_param(x: i32) {
    println!("the parameter value is : {}", x);
}

fn print_labeled_measurement(value: i32, uint_label: char) {
    print!("The measurement is: {value}{uint_label}");
}

fn statement_and_expressions() {
    // let x = (let y = 6);
    //  ^^^ expected expression, found `let` statement
    //  ^^^^^^^^^^^^^^ variable declaration using `let` is a statement
    // ^^^^^^^^^^^^^^ let` expressions in this position are unstable

    // let x = let y = 6; // let y = 6 statement does not return a value, so there isn’t anything for x to bind to.

    let y = {
        let x = 3; //  A new scope block created with curly brackets is an expression
        x + 12 // without semicolon
    };
    println!("the value of y is : {}", y);
}

// Functions with Return Values

fn return_ten() -> i32 {
    10
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; with semicolon
    /*
         --------            ^^^ expected `i32`, found `()`
      |    |
      |    implicitly returns `()` as its body has no tail or `return` expression
    8 |     x + 1;
      |          - help: remove this semicolon to return this value

         */
}

// ***** notes *****
/*
    - In function signatures, you must declare the type of each parameter
    - Statements are instructions that perform some action and do not return a value.
    - Expressions evaluate to a resultant value.
    - Expressions can be part of statements
    - Calling a function is an expression.
    - Calling a macro is an expression
    - Expressions do not include ending semicolons.
    - add a semicolon to the end of an expression, you turn it into a statement
*/
