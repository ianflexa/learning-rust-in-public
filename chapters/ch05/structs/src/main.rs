struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Using Tuple Structs Without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs Without Any Fields
struct AlwaysEqual;

// Ownership of Struct Data
struct User2 {
    active: bool,
    // username: &str,
    //      ^ expected named lifetime parameter
    // email: &str,
    //      ^ expected named lifetime parameter
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("ianflexa"),
        email: String::from("xx"),
        sign_in_count: 1,
    };

    // entire instance must be mutable
    user1.email = String::from("ianflexa.dev@gmail.com");

    let user2 = build_user(String::from("email@email.com"), String::from("user2"));

    // Creating Instances from Other Instances with Struct Update Syntax
    let user3 = User {
        email: String::from("another@email.com"),
        ..user1 // the syntax `..` specifies that the remaining fields not explicitly set
                // should have the same value as the fields in the given instance.
    };
    // we can no longer use user1 as a whole after creating user3
    // because the `String` in the username field of user1 was moved into user3.

    // Using Tuple Structs Without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields

    let subject = AlwaysEqual; // `unit-like` struct

}

// ***** notes *****

/*
    - `structs` are more flexible than `tuples` bc we don't have to rely on the order of the data to access the values of an instance
    - `bool` and `u64` implement the `Copy` trait
    - `unit-like` structs  behave similarly to ()
    - Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
    - It’s also possible for structs to store references to data owned by something else, but to do so requires the use of `lifetimes`
*/
