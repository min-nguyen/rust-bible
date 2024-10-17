use std::io;

// # Variables
//
// The term "variable" can be thought of as the value that it stores.
//


// ## Immutable Variables (Values)
//
// Variables are declared using `let .. = ..;`, and are immutable by default and so cannot have their values modified.
// Constants are declared using `const .. = ..;` and exactly the same as immutable variables, except:
//   1. They can be declared in sany scope including the global scope
//   2. They can only be set to constant expressions and not the result of a value that is only computable at runtime.
fn immutable_variables(){

    let x : i32 = 5; // x is immutable
    // x = 6;        // not allowed

    //   (The `let` clause shadows previous declared variables of the same name.
    let x : i32 = 6;  // shadows previous x = 5, and is allowed

    // Constants
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
}


// ## Mutable Variables  (Values)
//
// Variables can be declared mutable, allowing them their values to be modified.
//
// The One Rule of Mutability:
//  * Mutability is inherited in Rust: whether a value is mutable or not is decided by the variable that **owns** it.

fn mutable_variables(){
    let mut x : i32 = 5;  // x is mutable
    x = 6;    // allowed
    println!("The value of x is: {x}");
}
