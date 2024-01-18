fn main() {

    let mut s = String::new(); // `String` is implemented as a wrapper aroud a vector of bytes w some extra guarantees, restrictions, and capabilities.
    
    let data = "initial contents"; // reference

    let s = data.to_string(); // String Type

    let s = "initial contents".to_string(); // works directly

    let s = String::from("initial contents");

    // strings are UTF-8 encoded!

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating a String

    let mut s = String::from("hey");
    s.push_str(" you!"); // takes a string slice because we don't want to take ownership of the param

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // appending s2 contents to s1
    println!("{}", s2); //  we are able to use s2 here after appending its contents to s1

    let mut s = String::from("lo");
    s.push('l'); // `push` method takes a single `character` as a param and adds it to the String
    println!("{s}");

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("hey ");
    let s2 = String::from("you!");
    let s3 = s1 + &s2; // s1 has been moved here, so it can no longer be used. We are adding a `reference` of the second string to the first string

    /*
        The + operator uses the add method, whose signature looks something like this:

        fn add(self, s: &str) - > {}

        std lib: `add()` is defined using generics and associated types

        // Rust uses a `deref coercion`, which here turns &s2 into &s2[..]
     */

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // more complicated string combining

    // Indexing into Strings

    /*

        let s1 = String::from("wow");
        let h = s1[0]; //  using indexing syntax
                ^^^^^ `String` cannot be indexed by `{integer}`
        = help: the trait `Index<{integer}>` is not implemented for `String`

    */

    // Internal representation

    let hey = String::from("hey"); // Vec<u8>
    /*

        len will be 3, which means the vector storing the string "hey" is 3 bytes long, each letter takes 1 byte when encoded in UTF-8

    */

    let hello = String::from("Здравствуйте");
    /*
        len will be 24, "Здравствуйте" is 24 bytes long. Each Unicode sclar value in that string takes 2 bytes storage.

        let answer = &hello[0];

        if answer was valid code, the value will not be 3, because is encoded in UTF-8, so the first byte of 3 is 208. The only data that Rust has at byte index 0 is 208.
    */
    
    let hello = "Здравствуйте";
    
    // Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be
    // - a byte value
    // - a character
    // - a grapheme cluster
    // - a string slice
    
    let s = &hello[0..4]; // [] with a range to create a string slice containing particular bytes
    // ^s will be a `&str` that contains the first 4 bytes of the string.

    // Methods for Iterating Over Strings
    for c in s.chars() { // be explicit about what we want, in this case, chars
        println!("{c}");
    }

    for b in s.bytes() { // be explicit about what we want, in this case, bytes
        println!("{b}");
    }



}

// ***** notes *****
/*
    - `String` type: provided by Rust's std lib, is a growable, mutable, owned, UTF-8 encoded string type.
    - string slices `str`: usually seen in its borrowed form `&str`, are references to some UTF-8 encoded string data stored elsewhere. String literals, are stored in the program's binary and are therefore string slices.

    - A String is a wrapper over a Vec<u8>



*/
