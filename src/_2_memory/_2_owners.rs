// -----------------------------------------------
// # OWNERS
//
// The Three Rules of Ownership:
//  1. Each Value Has a Single Owner.
//     That is, each value is owned by a single variable.
//  2. When the Owner Goes Out of Scope, the Value Is Freed.
//       For values on the stack, this is managed trivially by the stack pointer.
//       For values on the heap, Rust automatically *drop*s the memory associated with the value.
//  3. Ownership Can Be Transferred (Moved).
//     The original variable becomes invalid, and the new variable becomes the owner of the data.
//
// A variable (i.e. its value) that is an owner of some data means it manages the data.
// This either means:
//   1) It is that data on the stack, and hence trivially manages itself.
//   2) It manages that data on the heap, and is an object of the form:
        // { ptr, len, capacity }
        //  a.  A pointer to the data
        //  b.  A length
        //  c.  A capacity
//

fn owner_example() {
  // x('s value) trivially owns 42 on the stack
  let x = 42;
  // y('s value) owns a Vec allocated on the heap
  let y: Vec<i32> = Vec::from([1,2,3]);
}
  // Informal Mental Model: what *COULD* happen:
  // STACK:
  // +-----------------------------------------+
  // | Stack Frame: owner_heap                 |
  // +-----------------------------------------+ 0x7ffeefbff4a0
  // | x: 42                                   | 4 bytes
  // +-----------------------------------------+ 0x7ffeefbff4a4
  // | y: Vec { ptr: 0x60004230,               | 8 bytes
  // |          len: 3,                        | 8 bytes
  // |          capacity: 12,                  | 8 bytes
  // | }                                       |
  // +-----------------------------------------+
  // HEAP:
  // +-----------------------------------------+ 0x60004230
  // | [1, 2, 3]                               | 12 bytes
  // +-----------------------------------------+


// -------------------------------------------------------------------------------------------------
// ## Value Scope and Dropping:
//
// Value Scope (rust-rfc#2094):
//   The scope of a value, which usually coincides with a variable name, is the span of time before it is freed and describes how long it is valid.
//   It begins when it is declared, and normally ends at the end of a block or function.
//    (Note: The scope of a reference can confusingly be called the value's lifetime)
//    (Note: The lifetime of a reference can confusingly be called the reference's lifetime, and has a different meaning)
//
// Dropping:
//   Each heap allocation must be paired with exactly one free when we’re done with it.
//   Heap memory is automatically freed once its owner goes out of scope, and is done via a special function "drop".
//
fn ownership_scope_example(){
    {
      // s (i.e. its value) owns a String allocated on the heap
      let s: String = String::from("hello"); // <<-- s is valid from this point forward
    } // <<-- s (i.e. its value) goes out of scope so is no longer valid.
      //      hence the data it manages is dropped.
}

// -------------------------------------------------------------------------------------------------
// ## Ownership Transfer: Moving, Copying, and Cloning Data
//
// Ownership transfer is an abstract concept that allows one to assign (=) a value stored in one variable to another to create a new owner.
// Depending on the type of value being assigned, the transfer can be labelled a MOVE, COPY, or CLONE.
// (By abstract, transferring ownership doesn't necessarily do anything in memory at all. A MOVE may perform a copy in memory, and a COPY may not copy anything. These details are entirely up to the compiler.)
//
//   a. A MOVE happens by default unless the assigned value implements the Copy trait.
//      This can be understood as:
//        - The value stored in the original variable is "moved" to the new variable.
//        - If the value involves managing other data (usually on the heap), that data is not duplicated.
//        - The original variable is invalidated, and the new variable owns the data.
//
//   b. A COPY happens if the assigned value implements the Copy trait.
//      Only applies to values not managing other data (usually on the heap).
//      This can be understood as:
//        - The value stored in the original variable is "copied" to the new variable.
//        - Both the original and new variables remain valid and own independent copies of the same value.
//
//   c. A CLONE happens if the assigned value is a result of clone().
//      Only applies to values that manage other data (usually on the heap).
//      This can be understood as:
//       - The value stored in the original variable is "copied" to the new variable,
//         but points to a newly allocated memory region.
//       - The data that the value pointed to is "cloned" to the new region.
//       - Both the original and new variables remain valid and own independent copies of the same data.

// -------------------------------------------------------------------------------------------------
// ## Mental Model: Ownership Transfer in practice.
//
// Ownership transfer is an abstract concept, and it is not usually productive to think of how it happens in memory.
// That is, transferring ownership doesn't necessarily do anything in memory at all.
// Semantically, all MOVE, COPY, and CLONE perform a memcpy i.e. an actual copy in memory.
// In practice, a memcpy won't happen unless necessary, and the optimiser can do anything as long as it does not change the program's behaviour.
//
//  - A MOVE may or may not copy the same value on the stack.
      // - For example, below COULD produce two copies of USER on the stack.
                        // struct User {
                        //   active: bool,
                        //   sign_in_count: u64,
                        // }
                        // fn structs_ownership(){
                        //   let user1 = User {
                        //     active: true,
                        //     sign_in_count: 1,
                        //   };
                        //   let user2 = user1;
                        // }
// - A COPY may or may not copy the same value on the stack.
            // - For example, below COULD reduce to one user on the stack, and both `user2` and `user1` refer to the same address on the stack
                        // #[derive(Clone, Copy)]
                        // struct User {
                        //   active: bool,
                        //   sign_in_count: u64,
                        // }
                        // fn structs_ownership(){
                        //   let user1 = User {
                        //     active: true,
                        //     sign_in_count: 1,
                        //   };
                        //   let user2 = user1;
                        // }
// - A CLONE may or may not copy the same value on the stack, and may or may not copy the same value on the heap.
//
// Having a mental model where every copy is a new value stored is fine, as long as you don't use this mental model to reason about performance.


//
// -------------------------------------------------------------------------------------------------
// [MOVE]
fn move_data(){
    {
        // s1('s value) manages a String on the heap
        let s1: String = String::from("hello"); // <<-- s1 is valid hereon.
        // s2('s value) manages a String whose ownership was moved from s1.
        let s2: String = s1; // <<-- s1 is no longer valid, and s2 is valid hereon
    } // <<-- Both s1 and s2 are out of scope (with s2 no longer valid).
      //      Only s2's String is dropped, as s1 does not manage any data.
}

// [COPY]
fn copy_data(){
    {
        // x('s value) trivially manages 5 on the stack
        let x: i32 = 5;      // <<-- x is valid hereon
        // y('s value) trivially manages 5 copied from x.
        let y: i32 = x;      // <<-- y is valid hereon
    } // <<-- Both x and y are out of scope (and no longer valid).
}

// [CLONE]
fn clone_data(){
    {
        // s1('s value) manages a String on the heap
        let s1: String = String::from("hello");   // <<-- s1 is valid hereon
        // s2('s value) manages a different String cloned from s1
        let s2: String = s1.clone();              // <<-- s2 is valid hereon
    } // <<-- Both s1 and s2 are out of scope (and no longer valid).
      //      Both of their managed data is dropped.
}

// -------------------------------------------------------------------
// ## OWNERSHIP TRANSFER in FUNCTION CALLS
//
// Passing a value as a function argument will transfer ownership exactly like when a value is assigned to a variable.
//
fn ownership_in_function_calls() {
    // s1 manages a String on the heap
    let s: String = String::from("hello"); // <<-- s is valid hereon
    // some_string manages the String whose ownership was moved from s.
    takes_ownership(s);       // <<-- some_string is valid and s is invalid hereon
    // <<-- some_string is out of scope and no longer valid; it's data is dropped,
    // The following line causes a compile-time error because s is no longer valid.
    // print!(s); // ERROR: `s` is invalid here because ownership was moved.

    // x manages 5
    let x: i32 = 5;                       // <<-- x is valid hereon.
    // some_integer manages 5 copied from x.
    makes_copy(x);           // <-- some_integer is valid
    // <<-- some_integer is out of scope and no longer valid

    // The following line works fine as x never became invalid.
    println!("{}", x);
} // <<-- Both x and s are out of scope (and invalid).
  //      Because s's ownership was already moved, there is nothing to drop.

fn takes_ownership(some_string: String) {   // <<-- some_string is valid hereon
  println!("{some_string}");
} // <<-- some_string goes out of scope (and is no longer valid).
  //      Its managed data is dropped.

fn makes_copy(some_integer: i32) {          // <<-- some_integer is valid hereon
  println!("{some_integer}");
} // <<-- some_integer goes out of scope (and is no longer valid).

// -------------------------------------------------------------------
// ### OWNERSHIP TRANSFER in FUNCTION RETURNS
//
// Returning a value from a function can also transfer ownership.
//
fn ownership_in_function_returns() {
    // s1 manages a String on the heap
    let s1 = gives_ownership();         // <<-- s1 is valid hereon
    // s2 manages a String on the heap
    let s2 = String::from("hello");     // <<-- s2 is valid hereon
    // s3 manages a String whose ownership was moved from s2.
    let s3 = takes_and_gives_back(s2); // <<-- s3 is valid and s2 is invalid hereon.
} // <<-- s3, s2, s1 go out of scope (and become invalid), and only s3's and s1's data are dropped.
  //      Because s2's ownership was moved, there is nothing to drop.

fn gives_ownership() -> String {
    // some_string manages a string on the heap
    let some_string = String::from("yours"); // <<-- some_string is valid hereon
    // some_string is returned and its ownership is moved
    some_string
} // <<-- some_string goes out of scope (and is no longer valid).
  //      Because its ownership was already moved, there is nothing to drop.

fn takes_and_gives_back(a_string: String) -> String { // <<-- a_string is valid hereon
    // a_string is returned and its ownership is moved
    a_string
}  // <<-- a_string goes out of scope (and is no longer valid).
  //       Because its ownership was already moved, there is nothing to drop.

