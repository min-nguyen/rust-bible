// -----------------------------------------------
// # SLICE REFERENCES
//
// A slice [T] is a contiguous sequence of data of type T stored in memory, being a slice of some collection rather than the entire collection itself.
//
// A slice reference &[T] (or fat pointer) is represented in the stack as two parts:
//       { ptr: ..., len: ... }
//    1. a pointer to the memory holding the contents of the slice
//    2. a slice length
// By including the slice length in the reference value, this lets us:
//    * Know the size of the slice at run-time that we can safely refer to.
//    * Know the size of the slice reference type &[T] at compile-time.

fn arrslice_example() {
  // arr is an array on the stack
  let arr: [i32; 5] = [1, 2, 3, 4, 5];

  // arr_ref is a normal reference to arr
  // its type includes the array's length, determined at compile-time.
  let arr_ref: &[i32; 5] = &arr;       // A normal reference to all of the array.

  // arr_sliceref is a slice reference from elements 2 to 4.
  // its value includes the slice's length, determined at run time.
  let arr_sliceref: &[i32] = &arr[1..4];

  // arr_wholesliceref is a slice reference to all of the array
  // its value includes the slice's length, determined at run time.
  let arr_wholesliceref: &[i32] = &arr[..];
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
    // | arr_sliceref:                      |  <--- `slice` is a slice reference to an array portion starting `arr[1]` to `arr[3]`.
    // |  { ptr: 0x7ffeefbff4a4, len: 3 }   |
    // +------------------------------------+ 0x7ffeefbff4c4
    // | wholeslice:                        |  <--- `wholeslice` is a slice reference to the entire array portion starting `arr[0]` to `arr[4]`.
    // |  { ptr: 0x7ffeefbff4a0, len: 5 }   |
    // +------------------------------------+

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
    // | 1                                        | <--- vec[0]
    // +------------------------------------------+ 0x60001238
    // | 2                                        | <--- vec[1]
    // +------------------------------------------+ 0x6000123C
    // | 3                                        | <--- vec[2]
    // +------------------------------------------+ 0x60001240
    // | 4                                        | <--- vec[3]
    // +------------------------------------------+ 0x60001244
    // | 5                                        | <--- vec[4]
    // +------------------------------------------+


// --------------------------------------------------------------------------------
// ## STRING SLICES (&str)
//
// A string slice (str) is essentially the type [u8], a sequence of UTF-8 encoded characters.
//  1. The type `str` has an unknown size at compile-time because it is designed to represent strings of arbitrary length.
//  2. Every variable in Rust must have a known size at compile time. This is true even for variables that refer to data
//     on the heap, because those variables' values are actually represented by pointers which have a known size.
// Hence it's not possible to generically allocate a value of type `str` because its size is not fixed.
// A string slice reference (&str) provides a way to refer to the actual slice (string data) without needing a specific size.
fn string_slices(){
  // Create an owner of type String pointing to data "hello" allocated on the heap
  let s = String::from("hello");

  let slice = &s[0..2];
  let slice = &s[..2];
}

// --------------------------------------------------------------------------------
// ### STRING SLICES AS PARAMETERS
//
// A parameter of type &str can accept both String references (&String) and string slices (&str).
// 1. If we choose to provide an argument of type &String, this is the same as a slice &str of the entire string.
// 2. If we choose to provide an argument of type &str, this could be any slice of the entire string.
fn get_first_word(s: &mut str) ->  &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

// --------------------------------------------------------------------------------
// ### STRING LITERALS AS SLICES
//
//  String literals are interpreted directly as a reference to a string slice.
//  The data of a string slice is hardcoded in the read-only section of the executable binary.
fn string_literals(){
  // Below will:
  //   1. Create a string slice "hello" that is a sequence of UTF-8 characters, which is then stored in the binary.
  //   2. Create a string slice reference "hello" that points to that string slice, stored on the stack.
  let s: &str = "hello";
  // The reference comprises of:
  //   1. a pointer to the start of string slice, which is a contiguous sequence 'h', 'e', 'l', 'l', 'o' stored on the binary.
  //   2, the length of the string slice on the binary
  let ptr = s.as_ptr();
  let len = s.len();
  print!("For string slice {s},  {len}");

  // The String::from method would copy the string literal data from the binary into the heap.
  let s = String::from(s);
}

// --------------------------------------------------------------------------------
// ## OTHER SLICES
//
fn i32_slices() {
  // Creates a fixed-size array stored on stack.
  let  xs:  [i32; 5] = [1, 2, 3, 4, 5];

  // Creates a reference to a slice that borrows the whole array.
  analyze_slice(&xs);
  // Creates a reference to a slice that borrows part of the array.
  analyze_slice(&xs[1..3]);

  fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
  }
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
