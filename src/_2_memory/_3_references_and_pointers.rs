mod references_and_pointers {

// -----------------------------------------------
// # REFERENCES AND BORROWING
// A variable is a REFERENCE to some value if it POINTS to the OWNER of that value.
//    This allows you to BORROW (read) that value without taking ownership of it, without making a copy,
//    and without preventing the original owner from accessing it once you're done.
//    Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
// A reference is represented in the stack as just one part:
//   - A pointer to an owner (or another reference)

//   [NAME     = VALUE]         [NAME     = VALUE]      [ IDX = VALUE ]
//   [ptr      = ...  ]  --->   [ptr      = ...  ] -->  [ 0   = 'h'   ]
//                              [len      = 5    ]      [ 1   = 'e'   ]
//                              [capacity = 5    ]      [ 2   = 'l'   ]
//                                                      [ 3   = 'l'   ]
//                                                      [ 4   = '0'   ]
//                                   ...
//      REFERENCE                   OWNER                 HEAP DATA

// There are two types of References:
//    1. Shared References (&). There can be as many of these
//    2. Mutable References (mut &)
// To access the underlying value, you can either:
//    1. Explicitly dereference it, which you write as *x.
//    2. Let the Rust compiler automatically redeference the reference, where you using it directly as x
// At any given time, you can have either one mutable reference or any number of immutable references.

// ## SHARED REFERENCES (borrowing)
// A shared reference (&x) can only read from the value it indirectly points to.
fn reference_example_1() {
    // Create a variable x
    let x: u32 = 10;
    // Create a reference to it
    let ref_x: &u32 = &x;
    // Explicitly deference and print out the value
    println!("x = {}", *ref_x);
    // Implicitly deference and print out the value
    println!("x = {}",  ref_x);
}

fn reference_example_2() {
    fn calculate_length(ref_s: &String) -> usize { // s is a reference to a String
        (*ref_s).len();  // Explicitly dereference it from &String to String to call the len() method.
        ref_s.len()      // Implicitly dereference it from &String to String to call the len() method.
    } // Here, s goes out of scope. But because it does not have ownership of the string data it indirectly refers to, that data is not freed.

    let s = String::from("hello");  // <<- Create an owner "s" that points to "hello" allocated on the heap
    let len = calculate_length(&s); // <<-- Create and pass a reference "&s" that points to "s" allocated the stack.
    println!("The length of '{s}' is {len}.");
}


// -------------------------------------------------------------------
// ## MUTABLE REFERENCES (mutable borrowing)
// A mutable reference (&mut) is allowed to mutate the value that it indirectly points to (not the address of the OWNER it points to).
// Mutable references have some rules:
//  1. Only mutable variables can have mutable references.
//     This makes sense: if the original owner is not able to change its data, then neither should any references to that data.
//  2. While a mutable reference is in scope, no new references can be declared, and no existing references (inc. the owner) can be used.
//     A reference's scope begins from when it is declared until the last time it is used.
//     In other words, only one variable may actively refer to a value.
fn mut_reference_example() {
    // mutable owner
    let mut s = String::from("hello");

    // mutable reference ref1_s is in scope
    let ref1  = &mut s;
    // let iref1 = &s;   <-- Not allowed, as ref1 is still being used
    // s.push_str("s");  <-- Not allowed, as ref1 is still being used
    ref1.push_str("s");
    // mutable reference  ref1_s will not be used after this point

    // mutable reference  ref2_s is in scope
    let ref2 = &mut s;
    ref2.push_str("s");
    // mutable reference  ref2_s will not be used after this point

    // owner s can be used again
    s.push_str("s");
}


// As variables can be references, we can also have combinations of (im)mutable variables that are (im)mutable references.
//   - `y: &i32`: Immutable variable y is an immutable reference to an i32 value.
//     You're not allowed to change anything.
//   - `mut y: &i32`: Mutable variable y is an immutable reference to an i32 value.
//     You're allowed to point y at a new memory location but not to change the contents of the memory it's pointing at.
//   - `y: &mut i32`: Immutable variable y is a mutable reference to an i32 value.
//     You're allowed to modify the contents of the memory y is pointing at, but not to change where it's pointing.
//   - `mut y: &mut i32`: Mutable variable y is a mutable reference to an i32 value.
//     You're allowed to modify the memory y is pointing at or to point y at a new location.

}


// -------------------------------------------------------------------
// ## DIFFERENCE BETWEEN REFERENCES AND POINTERS

// References and Pointers have the same underlying representation: both hold an address for some memory.
// Their difference is purely in their semantic interpretation,
// A reference variable provides a new name to an existing variable.
//      Interacting with a reference will refer to the underlying value it points to.
//      A reference does not need the dereferencing operator * to retrieve the value referenced.
// A pointer variable stores an arbitrary address (and can be null!).
//      Interacting with a pointer will be directly modifying the address it stores.
//      A pointer always needs to be dereferenced * to actually interact with the value it points to.
// In addition, references have rules enforced by the compiler:
//  1. References cannot outlive what they refer to
//  2. Mutable references cannot be aliased.
// Using pointers entails using unsafe Rust.

