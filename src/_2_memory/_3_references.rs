
// -----------------------------------------------
// # REFERENCES AND BORROWING
//
// A variable is a REFERENCE to some value if it points to the owner of that value.
//
// Creating a reference is called BORROWING, letting you borrow the value while:
//    - 1) not taking ownership of it
//    - 2) not making a copy,
//    - 3) not preventing the original owner from accessing it when done.
// Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the lifetime of that reference.
//
// A reference is represented in memory as just one part:
//   - { ptr : 0x... } A pointer to another variable or value.

fn refs_vs_owners() -> String{
    // x manages a string "golly" in memory (allocated on the heap)
    let x = String::from("golly");
    // y refers to, and borrows the value of, a string "gosh" in memory (allocated on the heap)
    let y = &String::from("gosh");

    // Allowed, as x manages a String so returning it will move ownership
    return x;
    // Not allowed, as y does not manage a String so cannot transfer ownership.
    // The value y points to (owned by an implicit variable) is dropped when the function exit.
    return *y;
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
    return arg // Allowed!
}
    // An Informal Mental Model of what *COULD* happen:
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
    // An Informal Mental Model of what *COULD* happen:
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
// # SHARED VS MUTABLE REFERENCES
//
// There are two types of References:
//    1. Shared References (&). This is read-only access. There can be many of these used in the same scope while the referenced data is not changed.
//    2. Mutable References (mut &). This is write and read access. While one is being used in scope, no other references can be used.
// At any given time, you can have either one mutable reference or any number of immutable references.
//
// Derefencing: to access the underlying value, you can either:
//    1. Explicitly dereference it, which you write as *x.
//    2. Let the Rust compiler automatically redeference the reference, where you using it directly as x

// -------------------------------------------------------------------
// ## SHARED REFERENCES (borrowing)
//
// A shared reference (ref : &T = &x) can only read from the value it borrows.
//
// The Rule for Shared References:
//  * While a shared reference is valid, the referenced data cannot change.
//    A reference's is valid from when it is declared until the last time it is used.
//    In other words, only one variable may actively refer to a value while it is being mutated.

fn shared_reference_example_1() {
    // x manages a value 10
    let x: u32 = 10;
    // ref_x and ref_y are both references to, and borrow the value of, x
    let (ref_x, ref_y) : (&u32, &u32) = (&x, &x);
    // Explicitly deference and print out the value
    println!("x = {}", *ref_x);
    // Implicitly deference and print out the value
    println!("x = {}",  ref_y);
}

fn shared_reference_example_2() {
    fn calculate_length(ref_s: &String) -> usize {
        // Explicitly dereference it from &String to String to call the len() method.
        (*ref_s).len();
        // Implicitly dereference it from &String to String to call the len() method.
        ref_s.len()
    } // <-- ref_s goes out of scope.
    //       Because it does manage any data, there is nothing to free.

    // s manages the string "hello" (on the heap)
    let s = String::from("hello");
    // below provides a reference to s as an argument, hence borrowing s's value,
    // and then sets len as the owner of a usize.
    let len = calculate_length(&s);
    println!("The length of '{s}' is {len}.");
}

// -------------------------------------------------------------------
// ## MUTABLE REFERENCES (mutable borrowing)
//
// A mutable reference (ref : &mut T = &mut x) is allowed to mutate the value that it indirectly points to (not the address of the OWNER it points to).
// Mutable references have some rules:
//  1. Only mutable variables can have mutable references.
//     This makes sense: if the original owner is not able to change its data, then neither should any references to that data.
//  2. While a mutable reference is in scope, no existing references (inc. the owner) can be used, and no new references can be declared
//     A reference's scope begins from when it is declared until the last time it is used.
//     In other words, only one variable may actively refer to a value while it is being mutated.
fn mut_reference_example() {
    // s is a mutable owner of "hello" in memory
    let mut s = String::from("hello");

    // ref1_s is a mutable reference in scope
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

// -------------------------------------------------------------------
// ## COMBINATIONS OF (IM)MUTABLE VARIABLES THAT ARE (IM)MUTABLE REFERENCES
//
// As variables themselves can be mutable, we can also have combinations of (im)mutable variables that are (im)mutable references.
// In other words, as well as modifying the referenced value, it is possible to modify what a reference points to.
fn mutability_in_references(){
    let mut x : i32 = 2;

    // r1 is a constant variable that is a shared reference to a (possibly mutable or immutable) i32 value.
    // You're not allowed to change what r1 points to, nor the content of the memory r1 points to.
    let r1 : &i32 = &x;

    // r2 is a mutable variable that is a shared reference to a (possibly mutable or immutable) i32 value.
    // You can point r2 to a new memory location, but you can't change the context of the memory r2 points to.
    let mut r2 : &i32 = &x;
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

