
// -----------------------------------------------
// # Slices and Slice References/Fat Pointers
// A slice refers to a specific slice of memory.

// --------------------------------------------------------------------------------
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
  print!("For string slice {s},  {len}");

  // We can also allocate a String that stores a string slice.
  // Below will create an owner of type String whose value points to data "hello" allocated on the heap
  let s_mutable: String = String::from(s);
}

//  The `str` type represents a sequence of UTF-8 encoded characters in Rust (stored in )
//  1. The type `str` has an unknown size at compile-time because it is designed to represent strings of arbitrary length.
//  2. Every variable in Rust must have a known size at compile time.
//     This is true even for variables that refer to data on the heap, because those variables' values are actually
//     represented by pointers which have a known size.
//  Hence it's not possible to generically allocate a value of type `str` because its size is not fixed.
//  This is in contrast to primitive types like `i32` or `f64` .

//  The `&str` type is a fat pointer, providing a way to refer to the actual slice (string data) without needing a specific size.
//  It has two components: a pointer to the start of the slice and the number of characters in the slice.
//  While values of type `str` can take on many sizes, values of type `&str` have a known fixed size.

//  String literals are interpreted directly as a reference to a string slice.
//  The data of a string slice is hardcoded in the read-only section of the executable binary.
