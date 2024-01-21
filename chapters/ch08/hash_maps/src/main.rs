use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new(); // HashMap<K, V>
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    // Accessing Values in a Hash Map

    let score = scores.get(&team_name).copied().unwrap_or(0); // The get method returns an Option<&V>;  `unwrap_or` to set score to zero if scores doesn't have an entry for the key.

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // `field_name` and `field_value` are invalid at this point

    // Updating a Hash Map

    /*
        - Replace the old value with the new value;
        - Keep the old value and ignore the new value;
        - Only adding the new value if the key doesn’t already have a value;
        - Combine the old value and the new value;
    
     */

    
    // Overwriting a Value
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // 10
    

    // Adding a Key and Value Only If a Key Isn’t Present

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(50); // The return value of the entry method is an enum called Entry

    // we want to check whether the key for the Yellow team has a value associated with it
    scores.entry(String::from("Yellow")).or_insert(50); // `entry` takes the key you want to check as a parameter.
    // `or_insert` returns a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() { // `split_whitespace` method returns an iterator over sub-slices, separated by whitespace,
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}

// ***** notes *****
/*
    - The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory.
    - Hash maps store their data on the heap
    - For types that implement the Copy trait, like i32, the values are copied into the hash map
    - For owned values like String, the values will be moved and the hash map will be the owner of those values

*/
