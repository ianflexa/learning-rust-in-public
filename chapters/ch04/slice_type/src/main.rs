fn main() {
    let mut s = String::from("Hello, world!");

    let word = bad_first_word(&s);

    s.clear(); // empties the String

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // String Slices
    let s = String::from("hello world");

    let hello = &s[0..5]; // reference to a portion of the String, specified in the extra [0..5] bit.
    let world = &s[6..11]; // [start_i..end_i]

    println!("{} {}", hello, world);

    // using the syntax
    let s = String::from("ian flexa");
    let len = s.len();

    let ian = &s[..3];
    let flexa = &s[4..len];
    println!("{ian} {flexa}");

    let flexa = &s[4..];
    println!("{flexa}");

    let slice1 = &s[0..len];
    let slice2 = &s[..];
    println!("{slice1} {slice2}");

    let s = String::from("ian flexa");

    let first = first_word(&s);
    println!("{first}");

    let second = second_word(&s);
    println!("{second}");

    // String Literals as Slices

    let s = "ian flexa"; // `s` is a slice pointing to that specific point of the binary. &str is an immutable reference.

    // Other Slices

    let a = [1, 2, 3, 4, 5];

    let slice = &a[..3];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn bad_first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert our String to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        // iterator over the array of bytes using the `iter` method
        // ^ returns a tuple (index, &element) == (i, &item)
        if item == b' ' {
            // b: byte literal syntax
            return i;
        }
    }

    s.len()
}

// fn first_word(s: &String) -> &str {
// A more experienced Rustacean would write the signature using `&str`,
// because it allows us to use the same function on both `&String` values and `&str` values.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn second_word(s: &String) -> &str {
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}

// ***** notes *****

/*
    - `iter` is a method that returns each element in a collection
    -  `enumerate` wraps the result of iter and returns each element as part of a tuple instead
    - A string slice is a reference to part of a String
    - the slice data structure stores the starting position and the length of the slice (ending_index - starting_index)
    - String slice range indices must occur at valid UTF-8 character boundaries
    - Using &str as arguments in signatures gives us the flexibility to take advantage of `deref coercions`
*/
