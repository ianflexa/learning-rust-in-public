#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // implementation block for Rectangle
    fn area(&self) -> u32 { // self in the signature
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }

    // Associated Functions
    fn square(size: u32) -> Self { // alias for the type that appears after the impl
        Self { width: size, height: size }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels", rect1.area());

    if rect1.width() {
        println!("The reactangle has a nonzero width; it is {}", rect1.width);

    }

    let rect2 = Rectangle {
        width: 10,
        height: 30
    };

    let rect3 = Rectangle {
        width: 30,
        height: 70
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // associated function
    let sqr1 = Rectangle::square(10);
    println!("The area of the square is {} square pixels", sqr1.area());
 
}


// ***** notes *****
/*
    - `methods` are similar to `functions`, but they're defined within the context of a `struct` (or `enum` or a  `trait object`)
    - `methods` first parameter is always `self`
    - All functions defined within an `impl` block are called `associated functions` because they’re associated with the type named after the `impl`.
    - `Associated functions` don't have self as their first parameter, bc they don't need an instance of the type to work with.
    - `Associated functions` are often used for constructors that will return a new instance of the struct
*/