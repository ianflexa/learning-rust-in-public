fn main() {
    // panic!
    // panic!("crash and burn");
    /*
        thread 'main' panicked at 'crash and burn', src/main.rs:3:5
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */

    
    // Using a `panic!` Backtrace
    // A `backtrace` is a list of all the functions that have been called to get to this point.
    let v = vec![1, 2, 3];

    v[99];
    /*
        thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:13:5

        
     */

    // RUST_BACKTRACE=1 cargo run
    /*
           0: rust_begin_unwind
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/std/src/panicking.rs:593:5
   1: core::panicking::panic_fmt
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:67:14
   2: core::panicking::panic_bounds_check
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/panicking.rs:162:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/slice/index.rs:261:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/alloc/src/vec/mod.rs:2675:9
   6: panic::main
             at ./src/main.rs:13:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/5680fa18feaa87f3ff04063800aec256c3d4b4be/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    
    
    
     */

}


// ***** notes *****
/*
    - Recoverable Errors: Report the problem to the user and retry the operation. i.e: `file not found` 
    - Unrecoverable Errors: bugs, so it will immediately stop the program. i.e: `trying to access a location beyond the end of an array`
    - panic!: print a failure message, unwind, clean up the stack, and quit.
    - `buffer overread`: In C, attempting to read beyond the end of a data structure is undefined behavior . You might get whatever is at the location in memory that would correspond to that element in the data structure

*/