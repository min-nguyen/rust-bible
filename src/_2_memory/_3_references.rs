
// -----------------------------------------------
// # REFERENCES AND BORROWING
//
// A variable (x : &T = &v) that is a REFERENCE to the value v of type T means it points to the owner of v.
//
//         let v : T  = ...;
//         let x : &T = &v;
//
// Creating a reference is called BORROWING, letting you borrow the value while:
//    - 1) not taking ownership of it
//    - 2) not making a copy,
//    - 3) not preventing the original owner from accessing it when done.
// A reference in Rust is guaranteed to point to a valid value for the lifetime of that reference.
// A reference is represented in memory as just one part:
//   - { ptr : 0x... } A pointer to another variable or value.

fn refs_vs_owners() -> String{
    // x manages a string "golly" in memory (allocated on the heap)
    let x: String = String::from("golly");
    // y refers to, and borrows the value of, a string "gosh" in memory (allocated on the heap)
    let y: &String = &String::from("gosh");

    // Allowed, as x manages a String so returning it will move ownership
    return x;   // Allowed!
    // Not allowed, as y does not manage a String so cannot transfer ownership.
    // The value y points to (owned by an implicit variable) is dropped when the function exit.
    return *y;  // Error: scope of y ends
}

fn refs_example_1(arg: &i32) -> &i32{

    // x manages 42
    let x = 42;
    // y is a reference to x
    let y = &x;
    // z is a reference to 7, not owned by an explicit variable.
    let z = &7;

    // We cannot return a reference to a local variable owned by the current function.
    // because the borrowed value does not live long enough.
    // return y; // ERROR: the value that y points to is dropped when the function exits.

    // We can return a reference to a variable owned outside of a function
    // because the borrowed value has a lifetime outside of this scope.
    return arg; // Allowed!
}
    // Informal Mental Model: what *COULD* happen:
    // STACK:
    // +-----------------------------------------+
    // | Stack Frame: ref_stack                  |
    // +-----------------------------------------+ 0x7ffeefbff4a0
    // | x:  42                                  | 4 bytes (stack-allocated integer)
    // +-----------------------------------------+ 0x7ffeefbff4a4
    // | y: 0x7ffeefbff4a0                       | 8 bytes (pointer to x on stack)
    // +-----------------------------------------+ 0x7ffeefbff4ac
    // | z: 0x7ffeefbff4b4                       | 8 bytes (pointer to temporary integer on stack)
    // +-----------------------------------------+ 0x7ffeefbff4b4
    // | Temporary Value: 7                      | 4 bytes (stack-allocated integer, not owned by any variable)
    // +-----------------------------------------+

fn ref_example_2(arg: &Box<i32>) -> &Box<i32> {
    // x manages 42 (allocated on the heap)
    let x = Box::new(42);
    // y is a reference to x
    let y = &x;

    // We cannot return a reference to a local variable owned by the current function.
    // because the borrowed value does not live long enough.
    // return &x; // Error
    // return y;  // Same error

    // We can return a reference to variable owned outside of a function
    // because the borrowed value has a lifetime outside of this scope.
    return arg;
}
    // Informal Mental Model: what *COULD* happen:
    // STACK:
    // +------------------------------------------+
    // | Stack Frame: ref_heap                    |
    // +------------------------------------------+ 0x7ffeefbff4a0 <--- x owns the heap-allocated data
    // | x: Box { ptr: 0x60001234,                | 8 bytes (pointer to 42 on heap)
    // |          len: ..,                        | 8 bytes
    // |          capacity: ..,                   | 8 bytes
    // | }                                        |
    // +------------------------------------------+ 0x7ffeefbff4a8 <--- y is a reference to x
    // | y: &Box { ptr: 0x7ffeefbff4a0 }          | 8 bytes (pointer to x on stack)
    // +------------------------------------------+
    //  HEAP:
    // +------------------------------------------+ 0x60001234
    // | 42                                       | 8 bytes
    // +------------------------------------------+

// -------------------------------------------------------------------
// ## Reference Lifetime and Dereferencing
//
// Reference Lifetime (rust-rfc#2094):
//    The lifetime of a reference is to the span of time in which it is used.
//    It begins when it is declared, and ends when it is last used.
//    (Note: The lifetime of a reference can confusingly be called the references's scope)
//    (Note: The scope of a value can confusingly be called the value's lifetime, and has a different meaning)
//
// Explicit and Implicit Dereferencing:
//    To access the underlying value of (x : &T), you can either:
//     1. Explicitly dereference it, written as *x.
//     2. Implicitly dereference it, written directly as x, whereby the Rust compiler automatically redeferences it.
//    Note that it is not always unambigious to the Rust compiler whether it should automatically deference a variable,
//    and so sometimes, we need to explicitly dereference it ourself

fn reference_lifetime_and_deferencing_example() {
    // x refers to a value 10
    let ref_x: &u32 = &10;     // <<-- start of ref_x's lifetime (as a reference)
    // Explicitly deference and print out the value
    println!("x = {}", *ref_x);
    // Implicitly deference and print out the value
    println!("x = {}",  ref_x); // <<-- end of ref_x's lifetime (as a reference)
} // <<-- end of ref_x's scope (as a value)


// -------------------------------------------------------------------
// ## Shared VS Mutable References
//
// There are two types of References:
//    1. Shared References (&) have read-only access.
//    2. Mutable References (mut &) have write and read access.
// At any given time, only one mutable reference can live or any number of immutable references can live.
//

// -------------------------------------------------------------------
// ### SHARED REFERENCES (borrowing)
//
// A shared reference (ref : &T = &x) can only read from the value it borrows.
//
// The Rule for Shared References:
//  * While a shared reference is alive (being used):
//    a. a mutable reference cannot be declared
//    b. the owner cannot change the referenced data

fn shared_reference_example() {
    // s is a mutable variable that manages the string "hello" (on the heap)
    let mut s = String::from("hello");

    // immut_ref_s1 and immut_ref_s2 are immutable references to s
    let (immut_ref_s1, immut_ref_s2) = (&s, &s); //<<-- start of ref_s1, ref_s2's lifetime (as a reference)

    // A mutable reference cannot be declared while ref_s1 or ref_s2 lives
    // let mut_ref_s : &mut String = &mut s;

    // The owner cannot change the referenced data while ref_s1 or ref_s2 lives
    // s.push('h');

    // Below is the last use of immut_ref_s1 and immut_ref_s2.
    println!("The length of '{}' is {}."
            , immut_ref_s1, (*immut_ref_s2).len()); // <<-- end of immut_ref_s1, immut_ref_s2's lifetime (as a reference)
}

// -------------------------------------------------------------------
// ### MUTABLE REFERENCES (mutable borrowing)
//
// A mutable reference (ref : &mut T = &mut x) can mutate the value that it borrows.
// (Note: It cannot mutate the address of what it points to).
//
// The Rules for Mutable References:
//  1. Only mutable variables can have mutable references.
//     I.e. if the owner cannot modify its data, then neither can any references.
//  2. While a mutable reference is alive (being used):
//     a. No new references can be declared.
//     b. No existing references can be used.
//     c. The owner cannot be used.
//  In other words, only one variable may actively refer to a value while it is being mutated.
fn mut_reference_example() {
    // s is a mutable owner of "hello" in memory
    let mut s = String::from("hello");

    // immut_ref_s is an immutable reference to s
    let immut_ref_s: &String = &s;

    // mut_ref_s is a mutable reference to s
    let mut_ref_s: &mut String  = &mut s;   // <<-- start of mut_ref_s's lifetime

    // No new references can be declared while mut_ref_s is alive
    // let new_immut_ref_s: &String = &s; // ERROR: mut_ref_s is used later

    // No existing references can be used while mut_ref_s is alive
    // print!("{immut_ref_s}"); // ERROR: mut_ref_s is still being used

    // The owner cannot be used while mut_ref_s is alive
    // s.push_str("s"); // ERROR: mut_ref_s is still being used

    // This marks the last usage of mut_ref_s
    mut_ref_s.push_str("s");         // <<-- end of mut_ref_s's lifetime

    // The owner s can be used again, as mut_ref_s is not used afterwards
    s.push_str("s");
}

// -------------------------------------------------------------------
// ## COMBINATIONS OF (IM)MUTABLE VARIABLES THAT ARE (IM)MUTABLE REFERENCES
//
// As variables themselves can be mutable, we can also have combinations of (im)mutable variables that are (im)mutable references.
// In other words, as well as modifying the referenced value, it is possible to modify what a reference points to.
fn mutable_variables_and_references(){
    let mut x : i32 = 2;

    // r1 is a constant variable that is a shared reference to a (possibly mutable or immutable) i32 value.
    // You're not allowed to change what r1 points to, nor the content of the memory r1 points to.
    let r1 : &i32 = &x;

    // r2 is a mutable variable that is a shared reference to a (possibly mutable or immutable) i32 value.
    // You can point r2 to a new memory location, but you can't change the context of the memory r2 points to.
    let mut r2 : &i32 = &x;
    print!("{r1}");
    r2 = &5;

    // r3 is a constant variable that is a mutable reference to a (necessarily mutable) i32 value.
    // You can change the contents of the memory r2 points to, but you can't change where r2 is pointing.
    let r3: &mut i32 = &mut x;

    // r4 is a mutable variable that is a mutable reference to a (necessarily mutable) i32 value.
    // You can point r4 to a new memory location, and you can change the contents of the memory r4 points at.
    let mut r4: &mut i32 = &mut x;
    let mut y: i32 =  6;
    r4 = &mut y;
    *r4  = 5;
}


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

