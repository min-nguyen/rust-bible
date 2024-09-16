// -----------------------------------------------
// # SLICE REFERENCES
//
// The array type [T; n] represents a contiguous sequence of data of type T.
// These have a known size n at compile-time so the contents are allocated on the stack, and we can directly use values of this type in a program.
//
// The slice type [T] is designed to represent a contiguous sequence of data of type T, being a slice of some collection (e.g. an array [T; n] or data structure like a Vec<T> or String) rather than the entire collection itself.
// These have an unknown size at compile-time: as every variable in Rust must have a known size, we cannot directly use values of this type.
//
// A slice reference (&[T]) provides a way to refer to a slice ([T]) without needing a specific size at compile-time.
// It is represented in memory as two parts:
//       { ptr: ..., len: ... }
//    1. a pointer to the memory holding the contents of the slice
//    2. a slice length, computed at run-time
// By including the slice length in the reference value, this lets us:
//    * Know the size of the slice reference type &[T] at compile-time.
//    * Know the size of the slice at run-time that we can safely refer to.
// The slice being referenced can either be on the stack or heap, depending on what the original datatype was.
//    - If the slice references an array or fixed-size data structure that is allocated on the stack, the data itself is on the stack.
//    - If the slice references data stored in a heap-allocated structure like a Vec<T> or String, the data is on the heap, but the slice's metadata (pointer and length) is still on the stack.

// An array [T; n] is a static sequence of elements of a known-size.
fn arrslice_example() {

  // Not allowed: arr of type [i32] is an arbitrary-sized array slice
  // let arr: [i32] = [1, 2, 3, 4, 5]; // Error: all local variables must have a known size at compile-time

  // Allowed: arr of type [i32;5] is an array slice of size 5 on the stack
  let arr: [i32; 5] = [1, 2, 3, 4, 5];

  // arr_ref is a normal reference &[i32; 5] to arr
  // its type includes the array's length, determined at compile-time.
  let arr_ref: &[i32; 5] = &arr;       // A normal reference to all of the array.

  // arr_sliceref is a slice reference &[i32]  to all of the array
  // its value includes the slice's length, determined at run time.
  let arr_sliceref: &[i32] = &arr[..];

  // arr_sliceref is a slice reference from elements 2 to 4.
  // its value includes the slice's length, determined at run time.
  let arr_sliceref: &[i32] = &arr[1..4];
}
    // Informal Mental Model: what *COULD* happen:
    //  STACK:
    // +------------------------------------+
    // | Stack Frame: slice_example         |
    // +------------------------------------+ 0x7ffeefbff4a0
    // | arr: [1, 2, 3, 4, 5]               |  <--- `arr` is an array stored on the stack
    // |    (address = 0x7ffeefbff4a0,      |
    // |             ..,                    |
    // |             ..,                    |
    // |             ..,                    |
    // |             ..0x7ffeefbff4b3 )     |
    // +------------------------------------+ 0x7ffeefbff4b4
    // | arr_ref:  0x7ffeefbff4a0           |  <--- `arr_ref` is a reference to the array.
    // |                                    |  The length of the array is known at compile-time, and need not be stored in memory.
    // +------------------------------------+ 0x7ffeefbff4bc
    // | arr_sliceref:                      |  <--- `wholeslice` is a slice reference to the entire array portion starting `arr[0]` to `arr[4]`.
    // |  { ptr: 0x7ffeefbff4a0, len: 5 }   |
    // +------------------------------------+ 0x7ffeefbff4c4
    // | arr_sliceref:                      |  <--- `slice` is a slice reference to an array portion starting `arr[1]` to `arr[3]`.
    // |  { ptr: 0x7ffeefbff4a4, len: 3 }   |
    // +------------------------------------+

// A Vec<T> is a dynamic representation of an array [T].
fn vecslice_example() {
  // vec manages a vector on the heap
  let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

  // arr_ref is a normal reference to vec
  // its value includes the vector's length, determined at run-time.
  let vec_ref: &Vec<i32>  = &vec;

  // vec_sliceref is a slice reference from elements 2 to 4.
  // its value includes the slice's length, determined at run time.
  let vec_sliceref: &[i32] = &vec[1..4];

  // vec_wholesliceref is a slice reference to all of the vector
  // its value includes the slice's length, determined at run time.
  let vec_wholesliceref: &[i32] = &vec[..];
}
    // Informal Mental Model: what *COULD* happen:
    //  STACK:
    // +------------------------------------------+
    // | Stack Frame: vecslice_example            |
    // +------------------------------------------+ 0x7ffeefbff498   <--- owner of heap-allocated vector
    // | vec: Vec {                               |
    // |   ptr: 0x60001234,                       | 8 bytes (pointer to vec[0] on heap)
    // |   len: 5,                                | 8 bytes
    // |   capacity: ..                           | 8 bytes
    // | }                                        |
    // +------------------------------------------+ 0x7ffeefbff4b0  <--- reference to the vector.
    // | vec_ref: &Vec {                          |
    // |   ptr: 0x7ffeefbff498                    | 8 bytes (pointer to `vec` on stack)
    // | }                                        |
    // +------------------------------------------+ 0x7ffeefbff4b8  <--- reference to the vector slice `vec[1]` to `vec[3]`.
    // | slice: {                                 |
    // |   ptr: 0x60001238,                       | 8 bytes (pointer to `vec[1]` on heap)
    // |   len: 3                                 | 8 bytes
    // | }                                        |
    // +------------------------------------------+ 0x7ffeefbff4c8 <--- reference to entire vector as a slice `vec[0]` to `vec[4]
    // | wholeslice: {                            |
    // |   ptr: 0x60001234,                       | 8 bytes (pointer to `vec[0]` on heap)
    // |   len: 5                                 | 8 bytes
    // | }                                        |
    // +------------------------------------------+ 0x7ffeefbff4d8
    //  HEAP:
    // +------------------------------------------+ 0x60001234
    // | 1, 2, 3, 4, 5                            | 8 bytes x 5 (vec[0] ... vec[4])
    // +------------------------------------------+ 0x60001244

// A string slice `str` is shorthand an array type [u8] **without** a known size.
// The `String` type is to [u8] what a Vec<T> is to [T].
fn strslice_example(){
  // s1('s value) is a reference to a string literal "hello" allocated on the stack
  // (which is hardcoded in and loaded from the read-only section of the executable binary).
  let s1: &str = "hello"; // i.e. let s1: &[u8; 5] = "hello";

  // s2('s value) owns a string "hello" allocated on the heap
  let s2: String = String::from("hello");

  // slice_s1('s value) is a reference to a string slice "he"  on the stack
  let slice_s1 = &s1[0..2];

  // slice_s2('s value) is a reference to a string slice "he"  on the heap
  let slice_s2 = &s2[..2];

  // The String::from method would copy the string literal data from the stack into the heap.
  let s = String::from(s1);
}

// --------------------------------------------------------------------------------
// ## MUTABLE SLICE REFERENCES
//
// The rules for mutable slice references are the same as for references.
//  ~ While a mutable reference lives, no other references can live, and the owner cannot be used.
// This is true even for two mutable slice references that refer to different parts of memory.
fn mutable_slices() {
  // xs is an array on the stack.
  let mut xs:  [i32; 5] = [1, 2, 3, 4, 5];

  // r1 is a mutable slice reference that borrows elements 2 to 4.
  let r1: &mut [i32] = &mut xs[1 .. 4];                // <<-- start of r1's lifetime
  println!("First element of the slice: {}", r1[0]);   // <<-- end of r1's lifetime

  // r2 is a mutable slice reference that borrows elements 5 onwards.
  let r2 = &mut xs[5 .. ];                 // <<-- start of r2's lifetime
  println!("First element of the slice: {}", r2[0]);   // <<-- end of r2's lifetime

  // A mutable reference cannot live at the same time as another reference, so below is not allowed:
  // println!("First element of the slice: {}", r1[0]); // Error: this would mean r2 was used during the lifetime of r1.

  // Allowed, as this is after r1 and r2's lifetime.
  xs = [1,3,4,4,32];
}

// --------------------------------------------------------------------------------
// ### STRING SLICES AS PARAMETERS
//
// A parameter of type &str can accept both String references (&String) and string slices (&str).
// 1. If we choose to provide an argument of type &String, this is the same as a slice &str of the entire string.
// 2. If we choose to provide an argument of type &str, this could be any slice of the entire string.
fn get_first_word(s: &mut str) ->  &str {
  let bytes: &[u8] = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}