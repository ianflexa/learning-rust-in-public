// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// Items in a parent modules can't use the private items inside the child modules.
// Items in child modules can use the items in their ancestor modules.
mod front_of_house {
    mod hostingPriv {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {} // making the module public doesn’t make its contents public.
    }
}

// defined in the crate root. `eat_at_restaurant` and `front_of_house` are siblings
pub fn eat_at_restaurant1() {
    // Absolute path
   // crate::front_of_house::hostingPriv::add_to_waitlist(); // imagine like a path:  /front_of_house/hosting/add_to_waitlist
                         // ^^^^^^^ private module

    // Relative Path
    //front_of_house::hostingPriv::add_to_waitlist(); // imagine like a path: front_of_house/hosting/add_to_waitlist
                    // ^^^^^^^ private module

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    
}


// Starting Relative Paths with super
fn deliver_order() {}

mod back_of_houseSuper {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // This is like starting a filesystem path with the .. syntax.
    }

    fn cook_order() {}
}


// Making Structs and Enums Public

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }

    pub enum Appetizer { // all of its variants are public
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast );
    // field `seasonal_fruit` of struct `Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");
    //  ^^^^^^^^^^^^^^ private field

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}

// Bringing Paths into Scope with the use Keyword

use front_of_house::hosting; // is similar to creating a symbolic link in the filesystem.

pub fn eat_at_restaurant3() {
    hosting::add_to_waitlist(); // `hosting` is now a valid name in that scope
}

// mod customer {
//       <------ need to declare inside the mod scope
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist()
//        ^^^^^^^ use of undeclared crate or module `hosting`
//     }
// }


mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

use std::collections::HashMap; // when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path

fn hashmap() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}


// bringin two items w the same name into scope.
/*
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
        // --snip--
        Result::Ok(())
    }

    fn function2() -> io::Result<()> {
        // --snip--
        Result::Ok(())
    }
 */


// Providing New Names with the `as` Keyword

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Result::Ok(())
}

fn function2() -> IoResult<()> {
    IoResult::Ok(())
}

// Re-exporting Names with `pub use`

mod front_of_house1 {
    pub mod hosting1 {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house1::hosting1; // re-exporting

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// Using Nested Paths to clean up large `use` lists
use std::{cmp::Ordering , i128};

use std::io::{self, Write}; // brings std::io and std::io::Write into scope.

// The Glob Operator

use std::collections::*; // brings all public items defined in a path into scope.

// ***** notes *****

/*
    - Modules let us organize code within a crate for readability and easy reuse.
    - Also allow us to control the privacy of items, because code within a module is private by default.
    - An `absolute path` is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal `crate`.
    - A relative path starts from the current module and uses self, super, or an identifier in the current module.
    - In Rust, all items (fn, methods, structs, enums, modules and constants) are private to parent modules by default
    - If you want to make a struct or fn private, you put it in a module.
    - Using super allows us to reference an item that we know is in the parent module.
    -  If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private.
    -  if we make an enum public, all of its variants are then public.
    - `use` only creates the shortcut for the particular scope in which the use occurs.
    - `Glob` can make it harder to tell what names are in scope and where a name used in your program was defined.
*/