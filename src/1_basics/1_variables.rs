use std::io;

// # Variables
fn variables(){

    // ## (Immutable) Variables
    // Variables are declared using `let .. = ..;`, and are immutable by default and so cannot be reassigned values:
    let x : i32 = 5; // x is immutable
    // x = 6;           // not allowed

    //   (The `let` clause shadows previous declared variables of the same name.
    let x : i32 = 6;  // shadows previous x = 5, and is allowed

    // ### Constants
    // Constants are declared using `const .. = ..;` and exactly the same as immutable variables, except:
    //   1. They can be declared in any scope including the global scope
    //   2. They can only be set to constant expressions and not the result of a value that is only computable at runtime.
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;

    // ### Mutable variables
    // Variables can be declared mutable, allowing them to be reassigned values:
    let mut x : i32 = 5;  // x is mutable
    x = 6;    // allowed
    println!("The value of x is: {x}");
}
