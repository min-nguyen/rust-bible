// -----------------------------------------------
// # OWNERS
//
// The Three Rules of Ownership:
//  1. Each Value Has a Single Owner.
//     That is, each value is owned by a single variable.
//  2. When the Owner Goes Out of Scope, the Value Is Freed.
//     For values on the stack, this is managed trivially by the stack pointer.
//     For values on the heap, Rust automatically *drop*s the memory associated with the value.
//  3. Ownership Can Be Transferred (Moved).
//     The original variable becomes invalid, and the new variable becomes the owner of the data.

// A variable that owns stack-allocated data means it stores the actual value.
fn owner_stack() {
  let x = 42; // x owns 42
}
  // An Informal Mental Model:
  //  STACK:
  // +--------------------------------+
  // | Stack Frame: owner_stack       |
  // +--------------------------------+ 0x7ffeefbff4a0
  // | x: 42                          |  <--- x owns the stack-allocated integer
  // +--------------------------------+

// A variable that owns heap-allocated data means it stores:
//  1.  A pointer to the data on the heap
//  and depending on the type of data, may also store:
//  2.  A length
//  3.  A capacity
fn owner_heap() {
  let x: Box<i32> = Box::new(42);       // x owns the heap-allocated Box
  let y: Vec<i32> = Vec::from([1,2,3]); // x owns the heap-allocated Vector
}
  // An Informal Mental Model:
  // STACK:
  // +-----------------------------------------+
  // | Stack Frame: owner_heap                 |
  // +-----------------------------------------+ 0x7ffeefbff4a0 <--- x owns the heap-allocated data
  // | x: Box { ptr: 0x60001234 }              | 8 bytes
  // +-----------------------------------------+ 0x7ffeefbff4a0 <--- x owns the heap-allocated data
  // | y: Vec { ptr: 0x60004230,               | 8 bytes
  // |          len: 3,                        | 8 bytes
  // |          capacity: 12,                  | 8 bytes
  // | }                                       |
  // +-----------------------------------------+
  // HEAP:
  // +-----------------------------------------+ 0x60001234
  // | 42                                      | 4 bytes
  // +-----------------------------------------+
  // | ...                                     |
  // +-----------------------------------------+ 0x60004230
  // | [1, 2, 3]                               | 12 bytes
  // +-----------------------------------------+

// -------------------------------------------------------------------------------------------------
// ## Ownership Scope and Drop:
//
// Each runtime request for heap allocation must be paired with exactly one free when we’re done with it.
// Heap memory is automatically freed once its owner goes out of scope  -- via a special function "drop".
// Stack memory is automatically cleaned up in the same way, but no explicit "drop" operation is needed -- the stack pointer is simply moved back.
// Often, the scope of coincides with the "syntaxy" scope, such as the end of a block or function.
//
fn ownership_scope(){
    {
        // The variable s owns a string on the heap
        let s: String = String::from("hello"); // <<-- s is valid from this point forward
    } // <<-- s is no longer valid
}

// -------------------------------------------------------------------------------------------------
// ## Ownership Transfer: Moving, Copying, and Cloning Data
//
// When assigning (=) a value from one variable to another, the transfer can be considered a MOVE, COPY, or CLONE.
// depending on the type of value being assigned.
//
//       a. A MOVE (of ownership) happens by default unless the assigned value implements the Copy trait.
//          On the stack, the value stored in the original variable is "moved" to the new variable.
//          The original variable is invalidated, and the new variable is now the owner of that data.
//          If the value involves heap data through a pointer, the heap data is not duplicated.
//       b. A COPY happens if the assigned value implements the Copy trait. This only applies to values that don't involve heap allocation.
//          On the stack, the value stored in the original variable is "copied" to the new variable.
//          If the value involves heap data through a pointer, the heap data is not duplicated.
//          Both the original and new variables remain valid, and they each own independent copies of the same data on the stack.
//          Since the Copy trait is only implemented for types not involving heap data, so heap data is duplicated.
//       c. A CLONE happens if the assigned value is a result of clone(). This only applies to values that involve heap allocation.
//          On the stack, the value stored in the original variable is "copied" to the new variable, but points to a newly allocated region on the heap.
//          On the heap, the data that the value pointed to is copied to the new region.
//          Both the original and new variables remain valid, and they each own independent copies of the same data on the heap.

// -------------------------------------------------------------------------------------------------
// [MOVE]
fn move_data(){
    {
        // The variable s1 owns a string on the heap
        let s1: String = String::from("hello"); // <<-- s1 is valid hereon.
        // The variable s2 owns the string on the heap moved from s1's ownership.
        let s2: String = s1; // <<-- s1 is no longer valid, and s2 is valid hereon
    } // <<-- Both s1 and s2 are out of scope (with s2 no longer valid), and s2's string is dropped.
}

// [COPY]
fn copy_data(){
    {
        // The variable x owns 5 on the stack
        let x: i32 = 5;      // <<-- x is valid hereon
        // The variable y owns 5 on the stack copied from x.
        let y: i32 = x;      // <<-- y is valid hereon
    } // <<-- Both x and y are out of scope (and no longer valid).
}

// [CLONE]
fn clone_data(){
    {
        // The variable s1 owns a string on the heap
        let s1: String = String::from("hello");   // <<-- s1 is valid hereon
        // The variable s2 owns a different string on the heap cloned from s1
        let s2: String = s1.clone();              // <<-- s2 is valid hereon
    } // <<-- Both s1 and s2 are out of scope (and no longer valid) and both of their heap-data is dropped.
}

// -------------------------------------------------------------------
// ## OWNERSHIP TRANSFER in FUNCTION CALLS
//
// Passing a value as a function argument will transfer ownership exactly like when a value is assigned to a variable.
//
fn ownership_in_function_calls() {
    // The variable s1 owns a string on the heap
    let s: String = String::from("hello"); // <<-- s is valid hereon
    // The argument "some_string" owns the string on the heap moved from s's ownership.
    takes_ownership(s);       // <<-- some_string is valid and s is invalid hereon
    // <<-- some_string is out of scope and no longer valid; it's data is dropped,
    // The following line would cause a compile-time error because s is no longer valid.
    // print!(s); // ERROR: `s` is invalid here because ownership was moved.

    // The variable x owns 5 on the stack
    let x: i32 = 5;                       // <<-- x is valid hereon.
    // The argument some_integer owns 5 on the stack copied from x.
    makes_copy(x);           // <-- some_integer is valid
    // <<-- some_integer is out of scope and no longer valid
    // The following line works fine as x never became invalid.
    println!("{}", x);
} // <<-- Both x and s are out of scope (and invalid).
  //      Because s's heap-data was already moved, there is nothing to drop.

fn takes_ownership(some_string: String) {   // <<-- some_string is valid hereon
  println!("{some_string}");
} // <<-- some_string goes out of scope (and is no longer valid) and its heap data is dropped.

fn makes_copy(some_integer: i32) {          // <<-- some_integer is valid hereon
  println!("{some_integer}");
} // <<-- some_integer goes out of scope (and is no longer valid).

// -------------------------------------------------------------------
// ### OWNERSHIP TRANSFER in FUNCTION RETURNS
//
// Returning a value from a function can also transfer ownership.
//
fn ownership_in_function_returns() {
    // The variable s1 owns a string on the heap
    let s1 = gives_ownership();         // <<-- s1 is valid hereon
    // The variable s2 owns a string on the heap
    let s2 = String::from("hello");     // <<-- s2 is valid hereon
    // The variable s3 owns a string on the heap moved from gives_owner()
    let s3 = takes_and_gives_back(s2); // <<-- s3 is valid and s2 is invalid hereon.
} // <<-- All s3, s2, s1 go out of scope (and become invalid), and only s3's and s1's data are dropped.
  //      Because s2's heap-data was moved,  there is nothing to drop.

fn gives_ownership() -> String {
    // The variable some_string owns a string on the heap
    let some_string = String::from("yours"); // <<-- some_string is valid hereon
    // The variable some_string is returned and its ownership is moved
    some_string
} // <<-- some_string goes out of scope (and is no longer valid).
//        Because its heap-data was already moved, there is nothing to drop.

fn takes_and_gives_back(a_string: String) -> String { // <<-- a_string is valid hereon
    // The variable a_string is returned and its ownership is moved
    a_string  // a_string is returned and moves out to the calling function
}  // a_string goes out of scope (and is no longer valid).
//    Because its heap-data was already moved, there is nothing to drop.

// -------------------------------------------------------------------------------------------------
// ## Important Note: Ownership Transfer in practice.
//
// Ownership transfer is an abstract concept, and it is not usually productive to think of how it happens in memory.
// That is, transferring ownership doesn't necessarily do anything in memory at all.
// Semantically, all MOVE, COPY, and CLONE perform a memcpy i.e. an actual copy in memory.
// In practice, a memcpy won't happen unless necessary, and the optimiser can do anything as long as it does not change the behaviour of your program.
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
// - A CLONE may or may not copy the same value on the heap, and may or may not copy the same value on the heap.
//
// Having a mental model where every copy is a new value stored is fine, as long as you don't use this mental model to reason about performance.

