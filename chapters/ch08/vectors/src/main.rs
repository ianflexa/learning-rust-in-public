fn main() {
    
    // createing a new vector V<T>

    let v: Vec<i32> = Vec::new(); // hold elements of `i32` type

    let mut v = vec![1, 2, 3]; // vec! macro

    // Updating a Vector

    v.push(4);
    v.push(5);
    v.push(6);

    // Reading Elements of Vectors

    let third: &i32 = &v[2]; // reference to the element at the index 2

    println!("the third element is {}", third);

    let third: Option<&i32> = v.get(2); // get -> Option<T>
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    /*
        let does_not_exit = &v[20]; 

        'main' panicked at 'index out of bounds: the len is 6 but the index is 20

        This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.
     */ 

    let does_not_exist = v.get(20);

    match does_not_exist {
        Some(does_not_exist) => println!(" .------.  {}", does_not_exist),
        None => println!("index 20 does not exist"),
    }

    /*
        let first = &v[0];
                    - immutable borrow occurs here
        v.push(10);
        ^^^^^^^^^^ mutable borrow occurs here
    
        println!("The first is: {}", first)
                                ---------- immutable borrow later used here
    
    
        Because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the elemnts to the new space.
    */

    // Iterating over the Values in a Vector

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50; // * dereference operator
    }

    /*
        If we attempted to insert or remove items in the for loop bodies in both examples above, we would get a compiler error. The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector.

    */

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    } // variants of an `enum` are defined under the same `enum` type

    let row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Float(4.2),
        SpreadsheetCell::Text(String::from("Hello"))
    ]; 

    // Dropping a Vector Drops Its Elements 

    {
        let v = vec![1, 2, 3];
        // do stuff w v
    } // <- v goes out of scope


 
}

// ***** notes *****

/*
    - Vectors allow you to store multiple values in a single data structure next to each other in memory.
    - Vectors can only store values that are the same type.
    - When use `enum` in a `V<T>`, Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. 

*/