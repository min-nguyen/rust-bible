// -----------------------------------------------
// # OWNERS AND OWNERSHIP
// A Variable is called an OWNER wrt to some value if it is the sole owner of that value.
// The concept of "owning" a value does not always imply that the value is stored on the heap.

// An OWNER (variable) of STACK-ALLOCATED data is simply that data itself:
fn owner_stack() {
    let x = 42; // x owns the stack-allocated integer
}
//  STACK:
// +--------------------------------+
// | Stack Frame: owner_stack       |
// +--------------------------------+ 0x7ffeefbff4a0
// | x: 42                          |  <--- x owns the stack-allocated integer
// +--------------------------------+

// An OWNER (variable) of HEAP-ALLOCATED data is represented in the stack as three parts:
//    1. a pointer to the heap memory holding the contents of the data
//    2. a length
//    3. a capacity being the total amount of heap memory
fn owner_heap() {
    let x = Box::new(42); // x owns the heap-allocated integer
}
//  STACK:
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

// RULES OF OWNERSHIP:
//      1. Each value has an owner
//      2. There can only be one owner at a time
//      3. When the owner goes out of scope, the value is dropped
// The point of ownership is to keep track of what code is using what data on the heap, minimize duplicate data on the heap, and clean up unused data on the heap.
// To support this, each runtime request for memory allocation must be paired with exactly one free which returns the memory when we’re done with it.
// In Rust, memory is automatically freed once the variable that owns it goes out of scope (by calling a special function "drop").

// -------------------------------------------------------------------------------------------------
// ### VARIABLE SCOPE

fn variable_scope(){ // A stack frame that allocates memory for local variable s.
  {
      // A variable of type String is an owner of a dynamic, growable string stored in the heap-allocated buffer.
      // String::from() takes a string slice, stores it on the heap, and returns an owner that points to it.
      let s: String = String::from("hello");  // <<-- s pushed onto stack and valid from this point forward
  }   // <<-- s is no longer valid, but is still on the stack until the function exits
}     // <<-- s is popped from stack

// -------------------------------------------------------------------------------------------------
// ### OWNERSHIP in VARIABLE ASSIGNMENTS: Copying, Moving, and Cloning Data

// [COPY]: Stack-Only Data
//   When assigning new variables to static data that is already stored on the stack, we can simply copy that data.
//   This is called a COPY.
//   (In contrast to assigning ownership for dynamic data, there is no difference between deep and shallow copying here).
fn copy_static_data(){ // A stack frame that allocates memory for local variables x and y is created.
    {
        // Create an owner x whose value is 5, and push onto the stack
        let x: i32 = 5;      // <<-- x is valid
        // Create an owner y and copy the value 5 in x to it, and push onto the stack.
        let y: i32 = x;      // <<-- y is valid and x is still valid
    } // <<-- x and y are no longer valid, but they are still on the stack until the function exits.
}
    //   The Copy Trait for Stack-Only Data:
    //    Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers.
    //    If a type implements Copy, variables that use it do not MOVE, but rather are trivially copied.
    //    Hence, variables implementing Copy are still valid after being assigned to another variable.
    //
    //    Here are some types that implement Copy:
    //      Integer types, such as u32.
    //      Boolean type, bool, with values true and false.
    //      Floating-point types, such as f64.
    //      Character types, char.
    //      Tuples if they only contain types that also implement Copy.

// [MOVE]: Heap-Only Data
//   When assigning new variables to dynamic data that is already stored on the heap, we perform a "shallow copy".
//   This is called a MOVE.
fn move_dynamic_data(){ // A stack frame that allocates memory for local variables s1 and s2 is created.
    {
        // Create an owner s1 of type String whose value points to data "hello" newly allocated on the heap, and push s1 onto the stack.
        let s1: String = String::from("hello");   // <<-- s1 is valid
        // Move the value of s1 to a new owner s2, and push s2 onto the stack. We do not copy the data on the heap that is pointed to.
        let s2: String = s1;                      // <<-- s2 is now valid and s1 is no longer valid
    } // <<-- s2 is no longer valid, so we free the memory it points to on the heap .
}

// [CLONE]: Heap-Only Data
//   If we _do_ want to actually copy heap data when assigning new variables to it, we perform a "deep copy".
//   This is called a CLONE.
fn clone_dynamic_data(){  // A stack frame that allocates memory for local variables s1 and s2 is created.
    {
        // Create an owner s1 of type String whose value points to "hello" newly allocated on the heap, and push s1 onto the stack.
        let s1: String = String::from("hello");   // <<-- s1 is valid
        // Create a new owner s2 whose value is distinct from s1's and points to a different "hello" newly allocated on the heap, and push s2 onto the stack.
        let s2: String = s1.clone();              // <<-- s2 is now valid and s1 is still valid
    } // <<-- s1 and s2 are no longer valid, so we free both of their memories (i.e. "hello" and "hello") they point to on the heap.
}

// -------------------------------------------------------------------
// ### OWNERSHIP in FUNCTION CALLS
// Passing a value as a function argument will transfer ownership exactly like when a value is assigned to a variable.
//   It will either 1) MOVE (for variables that own values on the stack pointing to heap-data)
//               or 2) COPY (for variables that own basic values on the stack),
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
// ### OWNERSHIP in FUNCTION RETURNS
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