// -----------------------------------------------
// # OWNERS
//
// A Variable is called an OWNER wrt to some value on the stack or heap if it is the sole owner of that value.
// The rules of ownership are as follow:
//  1. Each value has an owner
//  2. There can only be one owner at a time
//  3. When the owner goes out of scope, the value is dropped

// An OWNER of stack-allocated data is represented on the stack as that data directly.
fn owner_stack() {
  let x = 42; // x owns the stack-allocated integer
}
  //  STACK:
  // +--------------------------------+
  // | Stack Frame: owner_stack       |
  // +--------------------------------+ 0x7ffeefbff4a0
  // | x: 42                          |  <--- x owns the stack-allocated integer
  // +--------------------------------+

// An OWNER of heap-allocated data is represented on the stack as a pointer to that heap data.
// The owner has three parts:
  // 1. a pointer to the heap memory holding the contents of the data
  // 2. a length
  // 3. a capacity being the total amount of heap memory
fn owner_heap() {
  let x = Box::new(42); // x owns the heap-allocated integer
}
  // STACK:
  // +-----------------------------------------+
  // | Stack Frame: owner_heap                 |
  // +-----------------------------------------+ 0x7ffeefbff4a0 <--- x owns the heap-allocated data
  // | x: Box { ptr: 0x60001234,               | 8 bytes (reference to 42 on heap)
  // |          len: ..,                       | 8 bytes
  // |          capacity: ..,                  | 8 bytes
  // | }                                       |
  // +-----------------------------------------+
  // HEAP:
  // +-----------------------------------------+ 0x60001234
  // | 42                                      | 8 bytes
  // +-----------------------------------------+

// -------------------------------------------------------------------------------------------------
// ## Ownership Scope:
//
// Each runtime request for memory allocation must be paired with exactly one free which returns the memory when we’re done with it.
// In Rust, memory is automatically freed once the variable that owns it goes out of scope (by calling a special function "drop").
fn variable_scope(){ // A stack frame that allocates memory for local variable s.
  {
      // A variable of type String is an owner of a dynamic, growable string stored in the heap-allocated buffer.
      // String::from() takes a string slice, stores it on the heap, and returns an owner that points to it.
      let s: String = String::from("hello");  // <<-- s pushed onto stack and valid from this point forward
  }   // <<-- s is no longer valid, but is still on the stack until the function exits
}     // <<-- s is popped from stack

// -------------------------------------------------------------------------------------------------
// ## Ownership Transfer: Moving, Copying, and Cloning Data
//
// When assigning (=) a value from one variable to another, the transfer can be considered a MOVE, COPY, or CLONE.
// depending on the type of value being assigned.
//
//       a. A MOVE (of ownership) happens by default unless the assigned value implements the Copy trait.
//          On the stack, the value stored in the original variable is "moved" to the new variable.
//          If the value involves heap data through a pointer, the heap data is not duplicated.
//          The original variable is invalidated, and the new variable is now the owner of that data.
//       b. A COPY happens if the assigned value implements the Copy trait. This only applies to values that don't involve heap allocation.
//          On the stack, the value stored in the original variable is "copied" to the new variable.
//          Since the Copy trait is only implemented for types not involving heap data, so heap data is duplicated.
//          Both the original and new variables remain valid, and they each own independent copies of the same data on the stack.
//       c. A CLONE happens if the assigned value is a result of clone(). This only applies to values that involve heap allocation.
//          On the stack, the value stored in the original variable is "copied" to the new variable, but points to a newly allocated region on the heap.
//          On the heap, the data that the value pointed to is copied to the new region.
//          Both the original and new variables remain valid, and they each own independent copies of the same data on the heap.

// -------------------------------------------------------------------------------------------------
// ## An important note about Ownership Transfer in practice.
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
/// - A COPY may or may not copy the same value on the stack.
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
/// - A CLONE may or may not copy the same value on the heap, and may or may not copy the same value on the heap.
///
// Having a mental model where every copy is a new value stored is fine, as long as you don't use this mental model to reason about performance.


// -------------------------------------------------------------------------------------------------
// [MOVE]: Both Stack and Heap Data
fn move_data(){ // A stack frame that allocates memory for local variables s1 and s2 is created.
    {
        // Create an owner s1 of type String whose value points to data "hello" newly allocated on the heap, and push s1 onto the stack.
        let s1: String = String::from("hello");   // <<-- s1 is valid
        // Move the value of s1 to a new owner s2, and push s2 onto the stack. We do not copy the data on the heap that is pointed to.
        let s2: String = s1;                      // <<-- s2 is now valid and s1 is no longer valid
    } // <<-- s2 is no longer valid, so we free the memory it points to on the heap .
}

// [COPY] Stack-Only Data
fn copy_static_data(){ // A stack frame that allocates memory for local variables x and y is created.
    {
        // Create an owner x whose value is 5, and push onto the stack
        let x: i32 = 5;      // <<-- x is valid
        // Create an owner y and copy the value 5 in x to it, and push onto the stack.
        let y: i32 = x;      // <<-- y is valid and x is still valid
    } // <<-- x and y are no longer valid, but they are still on the stack until the function exits.
}

// [CLONE]: Heap-Only Data
fn clone_dynamic_data(){  // A stack frame that allocates memory for local variables s1 and s2 is created.
    {
        // Create an owner s1 of type String whose value points to "hello" newly allocated on the heap, and push s1 onto the stack.
        let s1: String = String::from("hello");   // <<-- s1 is valid
        // Create a new owner s2 whose value is distinct from s1's and points to a different "hello" newly allocated on the heap, and push s2 onto the stack.
        let s2: String = s1.clone();              // <<-- s2 is now valid and s1 is still valid
    } // <<-- s1 and s2 are no longer valid, so we free both of their memories (i.e. "hello" and "hello") they point to on the heap.
}

// -------------------------------------------------------------------
// ## OWNERSHIP TRANSFER in FUNCTION CALLS
// Passing a value as a function argument will transfer ownership exactly like when a value is assigned to a variable.
//   It will either MOVE or COPY, depending on whether the value implements the Copy trait.
fn ownership_in_function_calls() {
  let s: String = String::from("hello"); // s is in scope
  takes_ownership(s);       // s's value, a  on the stack that points to heap-data, is MOVED to a new variable "some_string"
                                        // ... and so s is no longer valid here.

  let x: i32 = 5;                         // x comes into scope
  makes_copy(x);            // x's value, an integer on the stack, is COPIED to a new variable "some_integer"
                                          // ... and so x is still valid here.
} // Here, x goes out of scope, and then s. But because s's value was already moved, nothing special happens.
fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// -------------------------------------------------------------------
// ### OWNERSHIP TRANSFER in FUNCTION RETURNS
// Returning a value from a function can also transfer ownership.
fn ownership_in_function_returns() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.
fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}
// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}