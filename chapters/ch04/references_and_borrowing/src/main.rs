fn main() {
    let s1 = String::from("hey");

    let length = calculate_length(&s1); // ampersands (&) represent `references`. Refering to s1 value without taking ownership of it
                                        // Because it does not own it, the value it points to will not be dropped when the reference stops being used.

    println!("The length of s1 is {length}");

    /*
       |     s     |          |    s1    |         | index | value |
       | ptr |  ---|----->    | ptr | ---|----->   |   0   |   h   |
                              | len | 3  |         |   1   |   e   |
                              | cpt | 3  |         |   2   |   y   |
    */

    // Mutable References

    let mut s1 = String::from("hey");

    change(&mut s1); // create a mutable reference

    // let r1 = &mut s1;
    // let r2 = &mut s1; // cannot borrow `s1` as mutable more than once at a time
    // println!("{} {}", r1, r2);

    {
        let r1 = &mut s1;
    } // r1 goes out of scope here, 

    let r2 = &mut s1; // so we can make a new reference with no problems.

    let mut s1 = String::from("hey");

    let r1 = &s1;
    let r2 = &s1; // multiple immutable references are allowed 
    // let r3 = &mut s1; // cannot borrow `s` as mutable because it is also borrowed as immutable

    // println!("{r1}{r2} {r3}")

    let mut s = String::from("hey");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References




}

fn calculate_length(s: &String) -> usize {
    // s is a `reference` to a `String`
    s.len()
} // Here, `s` goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

fn change_(some_s: &String) { // cannot borrow `*some_s` as mutable, as it is behind a `&` reference
                              // some_s.push_str("you!")
                              // ^^^^^^^^^^^^^^^^^^^^^^^ `some_s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

fn change(some_s: &mut String) {
    some_s.push_str("you!")
}

fn dangle() -> /*&String*/ String { // dangle returns a reference to a String
            // ^ expected named lifetime parameter
    let s = String::from("hey"); // `s` is a new string
    // &s // return a reference of `s`
    s 
} // `s` goes out of scope, and is dropped. Its memory goes away. Danger!

// ***** notes *****
/*
    - A reference is like a pointer in that it's an address we can follow to access the data stored at that address
    - A referece is guaranteed to point to a valid value of a particular type for the life of that referece
    - The opposite of `referencing` (&) is `dereferencing` (*)
    - We cannot borrow a `mut reference` more than once at a time. The benefit of this restriction is to prevent `data races`
    - `data races` happens when:
        1. Two or more pointers point to the same data at the same time
        2. At least one of the pointers is being used to write to the data
        3. There's no mechanism being used synchronize access to the data
    - We also cannot have a mutable reference while we have an immutable one to the same value.
    - `Dangling pointer` is a pointer that references a location in memory that may have been given to someone else

*/
