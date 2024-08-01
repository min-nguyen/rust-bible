

// -----------------------------------------------
// # OVERVIEW: Binary, Stack, and Heap Relationship
// -------------------------------------------------------------------------------------------------
// ### Binary:
//     The output of the compilation process is a binary executable. This binary contains machine code and sections for different types of data and code, including instructions for managing the stack and heap.
//     Binary Execution:
//         When you run the binary, the operating system loads it into memory and starts executing its instructions. The CPU begins executing the machine code, which includes setup for stack and heap usage.
// -------------------------------------------------------------------------------------------------
// ### Stack
// Usage: The stack can store values that have a known, fixed size (such as pointers) at compile-time. When the binary is executed, the stack is used for managing function calls, local variables, and control flow. The binary contains instructions that manipulate the stack pointer to allocate and deallocate memory as functions are called and return.
// Operations: As the program executes, the binary’s instructions manage the stack for function calls and local variable storage.
//  Every function call pre-allocates a stack frame, providing enough memory for its arguments and local variables.
//  As variables are introduced, their data is pushed onto the stack.
// Every function exit pops all the data in the stack frame off the stack.
//
// When foo() is called...
//     fn foo(x : uint32     <-- x is pushed onto stack
//          , y : &uint32) { <-- y is pushed onto stack
//         let z = x;        <-- z is pushed onto stack
//     }                     <-- z, y, then x are popped off of stack
//
//    +--------+
//    |        |
//    +--------+  \
//    | z      |  |
//    +--------+  |
//    | y      |  |
//    +--------+  |- pre-allocated stack memory for foo() at compile-time
//    | x      |  |
//    +--------+ /
///
// -------------------------------------------------------------------------------------------------
// ### Heap
//  Usage: The heap can store data for a specific requested amount of space, which may be unknown at compile time and may change at runtime. The binary contains instructions for managing heap allocations, which are used for dynamically allocated memory whose size and lifetime are not known at compile time. The heap is managed at runtime, with the binary including calls to the allocator to request and free memory.
//  Operations: The binary includes instructions to interact with the heap allocator.
//  The program requests memory from the heap for data structures whose size isn’t known until runtime.
// -------------------------------------------------------------------------------------------------
// ### Heap vs Stack Access:
// Stack access is faster because we never have to follow a pointer to get there, it is always relative to the top of the stack. This is true even if the data isn't always at the top:
    // 1. Local variables in a function have fixed offsets from the stack pointer or base pointer, making access straightforward and fast.
    // 2. The stack's contiguous memory layout means that when the CPU loads data from the stack into its cache, it often loads adjacent data as well,
    // 3. Simple Pointer Arithmetic: Accessing stack variables involves simple pointer arithmetic, which is efficient and fast.
// Heap access is slower because you have to follow a pointer to get there: contemporary processors are faster if they jump around less in memory.
// -------------------------------------------------------------------------------------------------
// ### Heap vs Stack Allocation:
// Stack pushing is faster because the allocator never has to search for a place to store new data; that location is always at the current stack pointer, which is trivially maintained (by simple pointer arithmetic).
// Heap allocation requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.


// -----------------------------------------------
// # OWNERS AND OWNERSHIP
// A Variable is called an OWNER wrt to some value if it is the sole owner of that value. This is typically wrt a value on the heap.
// An OWNER (variable) of heap-allocated data is  represented in the stack as three parts:
//    [ptr | len | capacity]
//   a pointer to the heap memory holding the contents of the data, a length, and a capacity being the total amount of heap memory.

    // Rules of ownership:
    //      1. Each value has an ownership
    //      2. There can only be one owner at a time
    //      3. When the owner goes out of scope, the value is dropped
    // Ownership addresses:
    //      1. Keeping track of what code is using what data on the heap
    //      2. Minimizing the amount of duplicate data on the heap
    //      3. Cleaning up unused data on the heap
    // To support dynamic data, we need to allocate an amount of memory on the heap that is unknown at compile-time.
    // This means:
    //     1. The memory must be requested from the memory allocator at runtime.
    //     2. We must return this memory to the allocator when we’re done with it.
    //   where we need to pair exactly one allocate with exactly one free.
    // In Rust, memory is automatically returned once the variable that owns it goes out of scope.
    //    When a variable goes out of scope, Rust calls a special function "drop" for us which returns the memory.

// -------------------------------------------------------------------------------------------------
// ### VARIABLE SCOPE
fn variable_scope(){ // A stack frame that allocates memory for local variables s_immutable and s_mutable is created.
  {
      // The type &str is for immutable strings.
      //   Aside: A variable of type &str is a fat pointer to a string literal of type str that was loaded and stored on the binary.
      // Create a **borrowed reference** "s_immutable" of type &str whose value points to data "hello" allocated on the heap.
      // The idea of references is **different** to owners.
      let s_immutable : &str = "hello";                   // <<-- s_immutable is valid from this point forward
      // The type String is for dynamic, growable strings.
      //   A variable of type String has ownership over the contents of the string which is stored in the heap-allocated buffer.
      // The method String::from takes a string slice, and requests the memory it needs from the heap.
      // Create an **owner** "s_mutable" of type String whose value points to data "hello" allocated on the heap
      let s_mutable: String = String::from(s_immutable);  // <<-- s_mutable is valid from this point forward
  }   // <<-- s_immutable and s_mutable are no longer valid, but they are still on the stack until the function exits
}
  // Side note: Strings are dedicated objects that manage their own heap-allocated memory:
             // they contain a pointer to some heap-data, but are not themselves pointers.
// -------------------------------------------------------------------------------------------------
// ### OWNERSHIP in VARIABLE ASSIGNMENTS: Copying, Moving, and Cloning Data
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
// [COPY]: Stack-Only Data
//   When assigning new variables to static data that is already stored on the stack, there is no difference between deep
//   and shallow copying here. Thus we can simply copy that data.
//   This is called a COPY.
fn copy_static_data(){  // A stack frame that allocates memory for local variables x and y is created.
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

// -----------------------------------------------
// # REFERENCES AND BORROWING
// A Variable is a REFERENCE to some value if it POINTS to the OWNER of that value.
//    This allows you to BORROW (read) that value without taking ownership of it, without making a copy,
//    and without preventing the original owner from accessing it once you're done.
//    Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
// A REFERENCE is represented in the stack as simply one part: a pointer to the OWNER (or to another pointer).
//    [ptr]
// Creating a REFERENCE is called BORROWING, which is done by using the ampersand (&).
//    The Rust compiler allows you to directly treat that REFERENCE as though it were the owner.
//    The Rust compiler AUTOMATICALLY HANDLES POINTER INDIRECTION. For example when you use methods on references,
//    we can call methods on a reference to a type as if you were calling them directly on the type itself.
// -------------------------------------------------------------------
// ## IMMUTABLE REFERENCES
// A REFERENCE is immutable by default, only able to read from the value it indirectly points to.
fn immutable_ref() {
    let s1 = String::from("hello"); // <<- Create an owner "s1" that points to "hello" allocated on the heap
    let len = calculate_length(&s1); // <<-- Create and pass a reference "&s1" that points to "s1" allocated the stack.
    println!("The length of '{s1}' is {len}.");
}
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()  // As s is a reference to s1, Rust will implicitly dereference it from &String to String
             // so that it can call the len() method.
} // Here, s goes out of scope.
  // But because it does not have ownership of the string data it indirectly refers to, that data is not freed.
// -------------------------------------------------------------------
// ## MUTABLE REFERENCES
// A MUTABLE REFERENCE is allowed to mutate the value that it indirectly points to (not the address of the OWNER it points to).
// Rules for Mutable References:
//  1. MUTABLE REFERENCES can only be created for OWNERS that are MUTABLE VARIABLES.
//     This makes sense: if the original owner is not able to change its data, then neither should any references to that data.
//  2. MUTABLE REFERENCES to a value can have no other references to that value.



// As variables can be references, we can also have combinations of (im)mutable variables that are (im)mutable references.
//   - `y: &i32`: Immutable variable y is an immutable reference to an i32 value.
//     You're not allowed to change anything.
//   - `mut y: &i32`: Mutable variable y is an immutable reference to an i32 value.
//     You're allowed to point y at a new memory location but not to change the contents of the memory it's pointing at.
//   - `y: &mut i32`: Immutable variable y is a mutable reference to an i32 value.
//     You're allowed to modify the contents of the memory y is pointing at, but not to change where it's pointing.
//   - `mut y: &mut i32`: Mutable variable y is a mutable reference to an i32 value.
//     You're allowed to modify the memory y is pointing at or to point y at a new location.


// -----------------------------------------------
// # Slices and Slice References/Fat Pointers
// A slice refers to a specific slice of memory.

// ## String Slices
// `str` and `&str` : String Slices and Slice References (Fat Pointers)
fn string_slices(){
    // Below will:
    //   1. Create a string slice "hello" that is a sequence of UTF-8 characters, which is then stored in the binary.
    //   2. Create a string slice reference "hello" that points to that string slice, stored on the stack.
    let s: &str = "hello";
    // The reference comprises of:
    //   1. a pointer to the start of string slice, which is a contiguous sequence 'h', 'e', 'l', 'l', 'o' stored on the binary.
    //   2, the length of the string slice on the binary
    let ptr = s.as_ptr();
    let len = s.len();
    print!("For string slice {s},  {len}")
}
// String Slices (`str`) for immutable strings is called a string slice, and is used to represent string literals.
// However, the references to string slices that we actually use (`&str`) are fat pointers stored on the stack.

//  The **string slice** `str` type represents a sequence of UTF-8 encoded characters in Rust.
//  1. **The type `str` has an unknown size at compile-time** because it is designed to represent strings of arbitrary length.
//      This is even though we know the size of a string literal, because the type `str` is not determined solely by the length of a single literal.
//  2. **Every variable in Rust must have a known size at compile time**
//      Even for variables whose values have a size that is only dynamically known (stored on the heap),
//      to access those values, we can only do so through pointers which have a known size.
//      Thus, being able to allocate a `str` at compile-time would imply that all values of type `str` have the same known siz, which isn't true.
//  3. **Hence it's not possible to generically allocate a value of type `str`** because its size is not fixed in the way primitive types like `i32` or `f64` are.

//  The **string slice reference** `&str` type are fat pointers, providing a way to refer to the actual slice (string data) without needing a specific size.
//    - The **only way you can safely represent a slice in memory** is through a **fat pointer** containing an address and size.
//      Hence `&str` has two components: a pointer to the start of the slice and the number of characters in the slice.
//      While values of type `str` can take on many sizes, values of type `&str` have a known fixed size.
//  Then,  **string literal data itself** of type `str` is then stored in the read-only section of the binary.
//    - Although that string data is represented using a dynamically sized type, its size is known at compile-time.
//      Hence we can hardcore this text directly as data into the  executable binary.

// String Literals: both String Slice References (Fat Pointers) AND String Slices
//  As A String Slice Reference (Fat Pointer):
//    - When declaring a string literal, Rust interprets it immediately as a reference to a string slice.
//      - The type of "hello" is &str, meaning Rust understands it is actually a reference to a string slice.
//      - The statement `let x: &str = "hello"` assigns this reference to the variable x stored on the stack, meaning we are borrowing the string slice.
//    -Note that things like `let x: &u32 = 5` are illegal because 5 is a `u32` value not a reference to a u32.
//  As The String Slice Data Itself:
//    - The value that the string slice reference points to is the actual string data.
//      - The value that the reference "hello" points to is the start of a sequence of characters 'h', 'e', 'l', 'l', 'o' stored directly in the executable binary.
//      - That reference, containining boththe pointer and the slice size, gives us information about the full slice.

fn main() {
    string_slices();
}
