fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels", area1(width1, height1));

    // Refactoring with Tuples

    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels", area2(rect1));

    // Refactoring with Structs: Adding More Meaning

    let rect2 = Rectangle{
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels", area3(&rect2));

    // Adding Useful Functionality with Derived Traits

   // println!("rect2 is {}", rect2); // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    println!("rect2 is {:?}", &rect2);
    println!("rect2 is {:#?}", &rect2); // {:#?} pretty

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    dbg!(&rect3);
}

fn area1(width: u32, height: u32) -> u32 { // it’s not clear anywhere in our program that the parameters are related.
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 { // this version is less clear: tuples don’t name their elements, 
    dimensions.0 * dimensions.1          // so we have to index into the parts of the tuple, making our calculation less obvious.
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// **** notes *****
/*
    - by default, the curly brackets tell `println!` to use formatting known as `Display`
    - ` dbg!` macro takes ownership of an expression prints the file and line number of where that `dbg!` macro call occurs in our code along, with the value, and returns ownership of the value

*/
