Ch07: Managing Growing Projects with Packages, Crates, nad Modules

Rust has a number of features that allow you to manage your code's organization, these features, sometimes collectively referred to as the `module system`, include:
    - Packages: A Cargo feature that lets you build, test, and share cates
    - Crates: A tree of modules that produces a library or executable
    - Modules and use: Let you control the organization, scope, and privacy of paths
    - Paths: A way of naming an item, such as a struct, function, or module


Packages and Crates:

- A `crate` is the smallest amount of code that the Rust compiler considers at a time.
- It can come in one of two formes: 
    - Binary crate: Programs you can compile to an executable that you can run (command-line program or a server). Each must have a `fn main`
    - Library crate: they don't have a `fn main`, and they don't compile ot an executable. They define functionality intended to be shared with multiple projects.

- The `create root` is a source file that the Rust compiler starts from and makes up the root module of your crate.

- A `package` is a bundle of one or more crates that provides a set of functionality.
- It contains a `cargo.toml` file that describes how to build those crates.


Defining Modules to Control Scope and Privacy

Modules Cheat Sheet
- `use` keyword brings a path into scope
- `pub` keyword make items public
- The compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.

