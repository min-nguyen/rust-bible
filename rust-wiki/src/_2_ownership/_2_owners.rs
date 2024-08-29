// -----------------------------------------------
// # OWNERS
//
// A variable (i.e. its value) that is an owner of some data means it manages the data.
// This either means:
//   1) It is that data on the stack, and hence trivially manages itself.
//   2) It manages that data on the heap, and is an object of the form:
//         { ptr, len, capacity }
//
// Three Rules of Ownership:
//  1. Each Value Has a Single Owner.
//       That is, each value is owned by a single variable.
//  2. When the Owner Goes Out of Scope, the Value Is Freed.
//       For values on the stack, this is managed trivially by the stack pointer.
//       For values on the heap, Rust automatically *drop*s the memory associated with the value.
//  3. Ownership Can Be Transferred (Moved).
//       The original variable becomes invalid, and the new variable becomes the owner of the data.
//
// Fourth Rule for Mutability:
//  4. The Owner is the Sole Decider of The Mutability of its Value.
//       This refers back Rust's rule of mutability i.e. that mutability is inherited in Rust.

fn owner_example() {
  // x1('s value) trivially owns 42 on the stack
  let x1 = 42;
  // xmut('s value) mutably owns an i32 on the stack, which is *copied* from another i32 (as i32 implements copy)
  fn add_one(mut xmut :   i32) -> i32{
    xmut += 1;
    return xmut;
  }
  // x2('s value) now owns 43 on the stack that was originally copied from x1 to xmut and modified.
  let x2: i32 = add_one(x1);
  print!("{0}", x2);

  // vec1('s value) is a Vec that owns a vector allocated on the heap
  let vec1: Vec<i32> = Vec::from([1,2,3]);
  // vecmut('s value) mutably owns an vector on the heap, which is *moved* from another vec (as vec does not implement copy)
  fn add_ones(mut vecmut : Vec<i32> ) -> Vec<i32>{
    vecmut.push(4);
    return vecmut;
  }
  // vec2('s value) is a Vec owns a vector on the heap that was moved from vec1 to vecmut and modified
  let vec2: Vec<i32> = add_ones(vec1);
  print!("{0}", vec2[4]);
}

// -------------------------------------------------------------------------------------------------
// ## Value Scope:
//
//  A value's scope is the region of code it is valid for.
//  It begins when it is declared, and normally ends at the end of a block or function.
//  When a value's scope ends, it is deallocated and any data it owns is dropped.
//
// Dropping:
//   Each heap allocation must be paired with exactly one free when we’re done with it.
//   Heap memory is automatically freed once its owner goes out of scope, and is done via a special function "drop".
//
//  [rust-rfc#2094]:
//    (Note: The scope of a reference can confusingly be called the value's lifetime)
//    (Note: The lifetime of a reference can confusingly be called the reference's lifetime, and has a different meaning)

fn ownership_scope_example(){

    { // s is a String object that owns a string allocated on the heap
      let s: String = String::from("hello"); // <<-- s is valid from this point forward
    } // <<-- s goes out of scope so is no longer valid.
      //      hence it is deallocated from the stack, and the data it manages is dropped from the heap.

    { // x is an int (trivially owning itself) on the stack
      let x : u32 = 5;        // <<-- x is valid from this point forward
    } // <<-- x goes out of scope so is no longer valid.
      //      hence it is deallocated from the stack.
}

// -------------------------------------------------------------------------------------------------
// ## Ownership Transfer: Moving, Copying, and Cloning Data
//
// Ownership transfer allows one to assign (=) a value stored in one variable to another to create a new owner.
// Depending on the type of value being assigned, the transfer can be labelled a MOVE, COPY, or CLONE.
//

//   a. A MOVE happens by default unless the assigned value implements the Copy trait.
//      This can be understood as:
//        - The value stored in the original variable is "moved" to the new variable.
//        - If the value involves managing other data (usually on the heap), that data is not duplicated.
//        - The original variable is invalidated, and the new variable owns the data.
fn move_data(){
  {
      // s1('s value), a String object, manages a string on the heap
      let s1: String = String::from("hello"); // <<-- s1 is valid hereon.
      // s2('s value) manages a string on the heap whose ownership was moved from s1.
      let s2: String = s1; // <<-- s1 is no longer valid, and s2 is valid hereon
  } // <<-- Both s1 and s2 are out of scope (with s2 no longer valid).
    //      Only s2's String is dropped, as s1 does not manage any data.
}

//   b. A COPY happens if the assigned value implements the Copy trait.
//      Only applies to values not managing other data (usually on the heap).
//      This can be understood as:
//        - The value stored in the original variable is "copied" to the new variable.
//        - Both the original and new variables remain valid and own independent copies of the same value.
fn copy_data(){
  {
      // x('s value) trivially manages 5 on the stack
      let x: i32 = 5;      // <<-- x is valid hereon
      // y('s value) trivially manages 5 copied from x.
      let y: i32 = x;      // <<-- y is valid hereon
  } // <<-- Both x and y are out of scope (and no longer valid).
}

//   c. A CLONE happens if the assigned value is a result of clone().
//      Only applies to values that manage other data (usually on the heap).
//      This can be understood as:
//       - The value stored in the original variable is "copied" to the new variable,
//         but points to a newly allocated memory region.
//       - The data that the value pointed to is "cloned" to the new region.
//       - Both the original and new variables remain valid and own independent copies of the same data.
fn clone_data(){
  {
      // s1('s value) manages a string on the heap
      let s1: String = String::from("hello");   // <<-- s1 is valid hereon
      // s2('s value) manages a different string on the heap cloned from s1
      let s2: String = s1.clone();              // <<-- s2 is valid hereon
  } // <<-- Both s1 and s2 are out of scope (and no longer valid).
    //      Both of their managed data is dropped.
}

// #### Ownership Transfer in Function Calls
//
// Passing a value as a function argument will transfer ownership exactly like when a value is assigned to a variable.
fn ownership_in_function_calls() {
    // s1 manages a string on the heap
    let s: String = String::from("hello"); // <<-- s is valid hereon
    // some_string manages the string whose ownership was moved from s.
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

// #### Ownership Transfer in Function Returns
// Returning a value from a function can also transfer ownership.
fn ownership_in_function_returns() {
    // s1 manages a string on the heap
    let s1 = gives_ownership();         // <<-- s1 is valid hereon
    // s2 manages a string on the heap
    let s2 = String::from("hello");     // <<-- s2 is valid hereon
    // s3 manages a string on the heap whose ownership was moved from s2.
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


// -----------------------------------------------
// ## Ownership Transfer: Full and Partial Moves (Elaborated more in `structs.rs`)
//
// When assigning a datatype from one variable to another, a full Move, Copy, or Clone applies for the entire structure.
//    let struct2 = struct1;
// A Move, Copy, or Clone applies to all its fields, and determines whether or not the parent variable is still valid.
// If a Move happens, the parent variable and its fields cannot be used afterwards.
//
// When assigning a component of a variable to another, a partial Move, Copy, or Clone applies for the entire structure.
//    let x = struct1.field;
// A Move, Copy, or Clone applies to just that field, and determines whether or not the component is still valid.
// If a Move happens, then parent variable cannot be used afterwards as a whole, but the unmoved parts can still be used.
//
// We can also combine by-move/copy/clone and by-reference pattern bindings at the same time, when destructuring a variable.
//    let Struct { field1, ref field2 } = x;
//  which is syntax sugar for:
//    let (field, field2) = (x.field1, &x.field2);
// A Move, Copy, or Clone applies to the reassigned field, and determines whether or not the component is still valid.
// A Borrow applies to the field we create a reference for, and so can still be used from the parent variable.
// If a Move happens, then parent variable cannot be used afterwards as a whole, but the unmoved parts (which includes
// the Borrow) can still be used.
//

fn partial_move_copy_clone_reference() {
  #[derive(Debug)]
  struct Person {
      first_name: String,
      last_name: String,
      age: u8,
      id: Box<u8>
  }

  let person = Person {
      first_name: String::from("Alice"),
      last_name: String::from("Smith"),
      age: 20,
      id: Box::new(0)
  };

  // `first_name` is referenced, `last_name` is moved, `age` is copied, and `id` is cloned.
  let first_name: &String = &person.first_name;  // reference
  let last_name: String = person.last_name;      // move
  let age: u8 = person.age;                      // copy
  let id: Box<u8>  = person.id.clone();          // clone

  println!("The person's first_name is {}", first_name);
  println!("The person's last_name is {}", last_name);
  println!("The person's age is {}", age);
  println!("The person's id is {}", id);

  // `person` cannot be used but their first_name, age, and id, can be used as they are not moved
  // println!("The person struct is {:?}", person); // Error! borrow of partially moved value.
  println!("The person's first_name from person struct is {}", first_name);
  println!("The person's age from person struct is {}", person.age);
  println!("The person's od from person struct is {}", person.id);
}

// -------------------------------------------------------------------------------------------------
// ## Mental Model: Ownership Transfer in practice.
//
// Ownership transfer is an abstract concept, and it is not usually productive to think of how it happens in memory. That is, transferring ownership doesn't necessarily do anything in memory at all. Semantically, all MOVE, COPY, and CLONE perform a memcpy i.e. an actual copy in memory. In practice, a memcpy won't happen unless necessary, and the optimiser can do anything as long as it does not change the program's behaviour. (A MOVE may perform a copy in memory, and a COPY may not copy anything. These details are entirely up to the compiler.) Having a mental model where every copy is a new value stored is fine, as long as you don't use this mental model to reason about performance.