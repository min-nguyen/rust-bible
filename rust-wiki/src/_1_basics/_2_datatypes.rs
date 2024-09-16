use std::io;

// # Data Types
// Every value in Rust is of a certain data type, where type annotations are specified with `:`.
// There are two data type subsets: scalar and compound.
fn data_types() {

  // ## Scalar Types
  // A scalar type represents a single value. Rust has four primary scalar types: integers, floats, Booleans, and characters.
  let byte: u8 = 0;
  let int: i64 = 20;
  let float: f64 = 20.0;
  let char: char = 'z';
  let bool: bool = true;

  // ## Compound Types
  // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

  // ### Tuples
  // A tuple is a fixed-size, heterogenous grouping of values.
  // Tuples are allocated on the stack, as they have a known number of elements.

  // We create a tuple by writing a comma-separated list of values inside parentheses, `(a, b, ...)`.
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // We can pattern match to destructure a tuple value:
  let (x, y, z) = tup;
  // We can access tuple elements directly by using `tuple.idx`:
  let tup_2: u8 = tup.2;

  // ### Arrays
  // An array is a fixed-size, homogeneous sequence of values.
  // Arrays are allocated on the stack, as they have a known number of elements.

  // We create an array by writing a comma-separated list of values inside square brackets, `[a, b ...]`.
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  // We can also initialise an array to contain the same value for each element by using `;`
  let arr: [i32; 5] = [3; 5]; // = [3, 3, 3, 3, 3]
  // We can access array elements directly by using `array[idx]`:
  let arr_2: i32 = arr[2];

  // ### Vectors
  // A vector is a dynamic, growable homogeneous sequence of values that can change size at runtime.
  // Vectors are allocated on the heap because their size can change and thus cannot be known at compile time.

  // We create a vector using the `vec!` macro:
  let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];

  // We can also create an empty vector and push elements to it:
  let mut empty_vec: Vec<i32> = Vec::new();
  empty_vec.push(10); // Adds 10 to the vector
  empty_vec.push(20); // Adds 20 to the vector

  // Vectors allow indexing similar to arrays:
  let third_element = vec[2]; // Accesses the third element (index 2)

  // ### Strings
  // Strings in Rust come in two primary types: `&str` and `String`.

  // #### String Slices (`&str`, essentially a byte slice reference `&[u8]` that guarantees valid UTF-8 characters)
  // A string slice is an immutable reference to a sequence of UTF-8 encoded bytes.

  // We can create a `&str` by:
  // 1. Referencing a hard-coding a "string literal", stored on the stack.
  let str_version: &str = "hello, world"; // String slice, typically immutable and stored in the binary or stack
  // 2. Referencing a dynamic String type, stored on the heap.
  let string_version : String = String::from("s");
  let str_version: &str = &string_version;

  // We can access elements in a string slice by indexing:
  let first_char = &str_version[0..1]; // Slices the first character, returns "h"

  // #### Strings (`String`, essentially a byte vector `Vec<u8>` that guarantees valid UTF-8 characters)
  // A `String` is a heap-allocated, growable, mutable string type.

  // We can create a `String` by:
  //  1. Converting a &str using the `String::from()` function
  let str_version: &str = "hello";
  let mut string_version : String = String::from(str_version);
  //  2. Owning a &str's data using the `ToOwned::to_owned()` method which works by cloning
  let mut string_version : String = str_version.to_owned();

  // We can mutate a `String`, adding more content:
  string_version.push_str(", world!"); // Appends ", world!" to the string

  // `String` can also grow dynamically at runtime:
  string_version.push('!'); // Adds a single character to the end


  // ### Invalid Element Acccess
  // Rust protects you against invalid element access by immediately exiting instead of allowing the memory access and continuing.
  let mut index = String::new();
  io::stdin()
      .read_line(&mut index)
      .expect("Failed to read line");
  let index: usize = index
      .trim()
      .parse()
      .expect("Index entered was not a number");
  let element = arr[index];                                       // terminates program eearly if index > arr.leng().
  println!("The value of the element at index {index} is: {element}"); // does not print if index > arr.len()

  // ### Mutable data
  // In Rust, mutability is always inherited. There is no intrinsic notion of mutability in the definitions of datatypes.
  // Hence, all components of a datatype inherit the mutability of the whole data type, which is decided by the owner that creates that data.
}