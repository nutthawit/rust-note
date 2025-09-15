use std::thread;

fn main() {
    // We must use `move` with closures passed to `thread::spawn`.

    // Compile error!
    // -------------------------------------------
    // let v = vec![1, 2, 3]; // even we have changed from `Vec<T>` to `i32` or type that implemented the `Copy` trait, this error also same.
    //
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {v:?}");
    // });
    //
    // handle.join().unwrap();
    // -------------------------------------------
    //
    // Because Rust can't tell how long the spawned thread will run so it doesn't know whether the reference to `v` will always be valid. took at below example:
    // -------------------------------------------
    // let v = vec![1, 2, 3];
    //
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {v:?}");
    // });
    //
    // drop(v); // oh no!
    //
    // handle.join().unwrap();
    // -------------------------------------------
    //
    // If Rust allowed us to run this code, the spawned thread has a reference to `v`, but the main tread immediately drops `v`, when the spawned thread start to execute, `v` is no longer valid.

    // Fix
    // -------------------------------------------
    let v = vec![1, 2, 3];

    // add `move` keyword
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
    // -------------------------------------------
    //
    // By adding the `move`, we force the closure to take ownership of the value rather than allow Rust to infer that is should borrow the value.
}
