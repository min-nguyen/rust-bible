
// -----------------------------------------------
// # REFERENCES AND BORROWING
// A variable is a REFERENCE to some value if it (possibly indirectly) POINTS to the OWNER of that value.
//    This allows you to BORROW (read) that value without taking ownership of it, without making a copy,
//    and without preventing the original owner from accessing it once you're done.
//    Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
// A REFERENCE is represented in the stack as just one part:
//   - { ptr : 0x... } A pointer to another variable or value.

// A REFERENCE to STACK-ALLOCATED data:
fn ref_stack() {
    let x = 42;  // x owns the stack allocated data 42
    let y = &x; // y is a reference to x
    let z = &7; // 7 is a temporary integer (not owned by any variable) stored on the stack
}
//  STACK:
// +--------------------------------+
// | Stack Frame: owner_stack       |
// +--------------------------------+
// | x:  42                         |  <---  x owns the stack allocated data 42.
// |    (address = 0x7ffeefbff4a0)  |
// +--------------------------------+
// | y: 0x7ffeefbff4a0              |  <---  y is a reference to x
// +--------------------------------+
// | z: 0x7ffeefbff4a8              |  <---  z is a reference to a temporary integer 7
// +--------------------------------+
// | Temporary Value: 7             |  <---   7 is a temporary integer (not owned by any variable) stored on the stack
// |    (address = 0x7ffeefbff4a8)  |
// +--------------------------------+

// An REFERENCE to HEAP-ALLOCATED data:
fn ref_heap() {
    let x = Box::new(42);  // x owns the heap-allocated integer
    let y = &x;           // y is a reference to x
}
//  STACK:
// +--------------------------------+
// | Stack Frame: ref_heap          |
// +--------------------------------+
// | x: Box { ptr: 0x1234,          |  <--- x owns the heap-allocated data
// |          len:..,               |
// |          capacity:.., }        |
// +--------------------------------+
// | y: &Box { ptr: 0x7ffeefbff4a0} |  <--- y is a reference to x
// +--------------------------------+
//  HEAP:
// +--------------------------------+
// | 0x1234: 42                     |  <--- heap-allocated integer
// +--------------------------------+

// -------------------------------------------------------------------
// # SHARED VS MUTABLE REFERENCES
// There are two types of References:
//    1. Shared References (&). This is read-only access. There can be many of these used in the same scope while the referenced data is not changed.
//    2. Mutable References (mut &). This is write and read access. While one is being used in scope, no other references can be used.
// To access the underlying value, you can either:
//    1. Explicitly dereference it, which you write as *x.
//    2. Let the Rust compiler automatically redeference the reference, where you using it directly as x
// At any given time, you can have either one mutable reference or any number of immutable references.

// ## SHARED REFERENCES (borrowing)
// A shared reference (ref : &T = &x) can only read from the value it indirectly points to.
// Shared references have some rules:
// 1. While a shared reference is in scope, the referenced data cannot change.
//    A reference's scope begins from when it is declared until the last time it is used.
//    In other words, only one variable may actively refer to a value while it is being mutated.
fn shared_reference_example_1() {
    // Create a variable x
    let x: u32 = 10;
    // Create a reference to it
    let (ref_x, ref_y) : (&u32, &u32) = (&x, &x);
    // Explicitly deference and print out the value
    println!("x = {}", *ref_x);
    // Implicitly deference and print out the value
    println!("x = {}",  ref_y);
}

fn shared_reference_example_2() {
    fn calculate_length(ref_s: &String) -> usize { // s is a reference to a String
        (*ref_s).len();  // Explicitly dereference it from &String to String to call the len() method.
        ref_s.len()      // Implicitly dereference it from &String to String to call the len() method.
    } // Here, s goes out of scope. But because it does not have ownership of the string data it indirectly refers to, that data is not freed.

    let s = String::from("hello");  // <<- Create an owner "s" that points to "hello" allocated on the heap
    let len = calculate_length(&s); // <<-- Create and pass a reference "&s" that points to "s" allocated the stack.
    println!("The length of '{s}' is {len}.");
}

// ## MUTABLE REFERENCES (mutable borrowing)
// A mutable reference (ref : &mut T = &mut x) is allowed to mutate the value that it indirectly points to (not the address of the OWNER it points to).
// Mutable references have some rules:
//  1. Only mutable variables can have mutable references.
//     This makes sense: if the original owner is not able to change its data, then neither should any references to that data.
//  2. While a mutable reference is in scope, no existing references (inc. the owner) can be used, and no new references can be declared
//     A reference's scope begins from when it is declared until the last time it is used.
//     In other words, only one variable may actively refer to a value while it is being mutated.
fn mut_reference_example() {
    // mutable owner
    let mut s = String::from("hello");

    // mutable reference ref1_s is in scope
    let ref1: &mut String  = &mut s;
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

// ## OTHER COMBINATIONS
// As variables can be references, we can also have combinations of (im)mutable variables that are (im)mutable references.
//   - `y: &i32`: y is a shared reference to an i32 value.
//     You're not allowed to change anything.
//   - `mut y: &i32`: y is a mutable reference to an i32 value.
//     You're allowed to point y at a new memory location but not to change the contents of the memory it's pointing at.
//   - `y: &mut i32`: Immutable variable y is a mutable reference to an i32 value.
//     You're allowed to modify the contents of the memory y is pointing at, but not to change where it's pointing.
//   - `mut y: &mut i32`: Mutable variable y is a mutable reference to an i32 value.
//     You're allowed to modify the memory y is pointing at or to point y at a new location.

// -------------------------------------------------------------------
// ## DIFFERENCE BETWEEN REFERENCES AND POINTERS

// References and Pointers have the same underlying representation: both hold an address for some memory.
// Their difference is purely in their semantic interpretation,
// A reference variable stores the address of an existing variable (or unnamed value) that is already allocated in memory.
//      Interacting with a reference will refer to the underlying value of that address.
//      A reference does not need the dereferencing operator * to retrieve the value referenced, but is automatically determined by Rust (via pointer indirections where necessary).
// A pointer variable stores any arbitrary address (and can be null!).
//      Interacting with a pointer will be directly modifying the address it stores.
//      A pointer always needs to be dereferenced * to actually interact with the value it points to.
// In addition, references have rules enforced by the compiler:
//  1. References cannot outlive what they refer to
//  2. Mutable references cannot be aliased.
// Using pointers entails using unsafe Rust.

