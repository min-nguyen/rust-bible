
// -----------------------------------------------
// # REFERENCES
//
// A variable (x : &T = &v) that is a reference to the value v of type T means it points to the owner of v.
//
//         let v : T  = ...;
//         let x : &T = &v;
//         (We could also write `let ref x = v`, but the intended use of `ref` is for pattern matching.)
//
// Creating a reference is called borrowing, letting you borrow the value while:
//    1) Not taking ownership of it
//    2) Not making a copy,
//    3) Not preventing the original owner from accessing it when done.
//
// A reference is represented in memory as just one part:
//   - { ptr : 0x... } A pointer to another variable or value.
//
struct IntBox(i32);  // A datatype that does not implement Copy (unlike i32)

fn refs_vs_owners() -> (IntBox, String){
    // x is (owns) IntBox(42)
    let x: IntBox = IntBox(42);
    // y is a reference to x
    let xref: &IntBox = &x;
    // z is a reference to IntBox(7), not owned by an explicit variable.
    let intbox_ref: &IntBox = &IntBox(7);

    // s is a String object that owns a string (allocated on the heap)
    let s: String = String::from("golly");
    // s is a String object that owns a string (allocated on the heap)
    let sref: &String = &s;
    // y is a reference to, and borrows the value of, a string "gosh" in memory (allocated on the heap)
    let string_ref: &String = &String::from("gosh");

    // Allowed, as x and s are owners, so returning it will move ownership
    return (x, s);
    // Not allowed, as i32_ref and string_ref do not own anything, so cannot transfer ownership (unless the referents implement Copy).
    return (*intbox_ref, *string_ref);   // Error: scope of xref's and sref's borrowed value will end upon function
}

// -------------------------------------------------------------------
// ## Dereferencing
//
// Explicit and Implicit Dereferencing:
//    To access the underlying value of (x : &T), you can either:
//     1. Explicitly dereference it, written as *x.
//     2. Implicitly dereference it, written directly as x, whereby the Rust compiler automatically redeferences it.
//    Note that it is not always unambigious to the Rust compiler whether it should automatically deference a variable,
//    and so sometimes, we need to explicitly dereference it ourself

fn reference_deferencing_example() {
    // x refers to a value 10
    let ref_x: &u32 = &10;
    // Explicitly deference and print out the value
    println!("x = {}", *ref_x);
    // Implicitly deference and print out the value
    println!("x = {}",  ref_x);
}

// -------------------------------------------------------------------
// ## Reference Lifetime
//
//    A reference's lifetime is a (named) region of code that the compiler guarantees to point to a valid value.
//    It begins when it is declared and ends when it is no longer used, which must be before its pointed value goes out of scope.
//    A reference's lifetime hence must not outlive the scope of its value; i.e. while references to an object exist, the object cannot be destroyed.
//
// Syntax Sugar for Lifetimes:
//   The lifetime of a reference is usually implicit and hidden by syntax sugar.
//   It is only necessary to explicitly annotate references by lifetimes (like xref : &'a T) to prevent dangling pointers and data races,
//   in which case this also requires generic parameters (not talked about here).

// Below shows when reference lifetimes prevent dangling pointers.
fn refs_lifetime_example(arg: &i32) -> &i32{
    // x is (owns) 42
    let x: i32 = 42;
    // xref is a reference to x
    let xref: &i32 = &x;

    // We can return this reference, as the scope of the value it refers to is outside of this function.
    return arg; // Allowed!

    // We cannot return this reference, as this means it lives past the scope of the value it refers to.
    return xref; // ERROR: the value that xref points to is dropped when the function exits.
}

// Below elaborates on what reference lifetimes would refer to as regions of code.
// Note: The syntax 'a { ...} are simply named regions (which lifetime names would coincide with).
//       No explicit reference lifetimes (i.e. the generic parameter <'a> or the type &'a T) are actually used.
fn refs_lifetime_elaborated(){
    // Example 1:
    // Each let statement implicitly introduces a scope.
    let x: i32 = 42;     // x is (owns) 42
    let y: &i32 = &x;    // y is a reference to x
    let z: &&i32 = &y;   // z is a reference to y which is a reference to x
    // which desugars to:
    'a: { // lifetime that x is declared within is 'a
        let x: i32 = 0;
        'b: {
            // lifetime used for y is 'b because that's good enough to reference 'a.
            let y: &i32 = & x;
            'c: {
                //  lifetime used for z is 'c because that's good enough to reference y with lifetime 'b.
                let z: &&i32 = &y; // "a reference to a reference to an i32"
            }
        }
    }

    // Example 2:
    // Passing references to outer scopes will cause Rust to infer a larger lifetime:
    let a: i32 = 0;
    let b: &&i32;
    let c: &i32 = &x;
    b = &c;
    // which desugars to:
    'a: {
        let a: i32 = 0;
        'b: {
            // lifetime used for b is 'b
            let b: &&i32;
            'c: {
                // lifetime used for c is 'b because it must live long enough for to be used by reference b with lifetime 'b.
                let c: &i32 = &a;
                b = &c;
            }
        }
    }
}

// Below shows some subtle cases about reference lifetimes.
fn refs_lifetimes_subtleties(){

    // Below is fine:
    let mut x: i32 = 10;
    let xref: &i32 = &x;
    // last usage of reference x:
    println!("{}", xref);
    // this is OK, as xref is no longer used
    x = 5;

    // Below is not fine:
    #[derive(Debug)]
    struct X<'a>(&'a i32);
    impl Drop for X<'_> { fn drop(&mut self) {}  }
    let mut x: i32 = 5;
    // x is a value that contains an immutable reference to data
    let xrefcontainer: X<'_> = X(&x);
    println!("{:?}", xrefcontainer);
    // this is not okay as `xrefcontainer` is a value that implements drop (only called when it goes out of scope):
    //  x = 6; // ERROR: we haven't finished using the immutable reference to `data` that `x` contains.

}   // <-- drop(x) is called here

// -------------------------------------------------------------------
// ## Shared vs Mutable References
//
// There are two types of References:
//    1. Shared References (&) have read-only access.
//    2. Mutable References (mut &) have write and read access.
// At any given time, only one mutable reference can live or any number of immutable references can live.
//
// ### Shared References
//
// A shared reference (ref : &T = &x) can only read from the value it borrows.
//
// The Rule for Shared References:
//  * While a shared reference is alive (being used):
//    a. a mutable reference cannot be declared
//    b. the owner cannot change the referenced data
//
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
//
// ### Mutable References
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
//
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
// ## (Im)mutable Variables that are (Im)mutable References
//
// As variables can be mutable, we can also have combinations of (im)mutable variables that are (im)mutable references.
// In other words, we can both modify what a reference points to as well as modify what the referenced data.
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

