fn main() {
    let number = 10;

    if number < 20 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Rust will not automatically try to convert non-Boolean types to a Boolean
    //    if number {
    //      ^^^^^^ expected `bool`, found integer
    //    println!("number ten");
    //    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" };
    //                                        ^^^^^ expected integer, found `&str`

    println!("The value of number is: {number}"); 


    // Repetition with Loops
    // loop {
    //     println!("again!"); // infinite loop
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{counter}");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;

    'counting_up: loop { // loop with a label
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("while loop has ended!");


    let a = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() { // Range, provided by the standard library,
        println!("{number}!");
    }

}

// ***** notes *****

/*
    - You must be explicit and always provide if with a Boolean as its condition.
    - values that have the potential to be results from each arm of the if must be the same type
*/
