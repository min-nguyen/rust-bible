use std::io;

// # Data Types
// Every value in Rust is of a certain data type, where type annotations are specified with `:`.
// There are two data type subsets: scalar and compound.
fn data_types() {
  // ## Scalar Types
  // A scalar type represents a single value. Rust has four primary scalar types: integers, floats, Booleans, and characters.
  let i: i64 = 20;
  let f: f64 = 20.0;
  let c: char = 'z';
  let b: bool = true;

  // ## Compound Types
  // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

  // ### Tuples
  // A tuple groups values with a variety of types into one compound type.
  // We create a tuple by writing a comma-separated list of values inside parentheses, `(a, b, ...)`.
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // We can pattern match to destructure a tuple value:
  let (x, y, z) = tup;
  // We can access tuple elements directly by using `tuple.idx`:
  let tup_2: u8 = tup.2;

  // ### Arrays
  // An array groups values with the same type. Arrays are allocated on the stack, as they have a fixed number of elements.
  // We create an array by writing a comma-separated list of values inside square brackets, `[a, b ...]`.
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  // We can also initialise an array to contain the same value for each element by using `;`
  let arr: [i32; 5] = [3; 5]; // = [3, 3, 3, 3, 3]
  // We can access array elements directly by using `array[idx]`:
  let arr_2: i32 = arr[2];

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

}