// -----------------------------------------------
// # SLICE REFERENCES
// A slice [T] is a contiguous sequence of data of type T stored in memory, being a slice of some collection rather than the entire collection itself.
// A slice reference &[T] is a reference to that slice. Many times, people talk about &[T] as the slice.

// A slice reference is a fat pointer and has two parts: a pointer and a slice length.
//      [ADDR  (variable)      VALUE                  ]           [ADDR    VALUE  ]
//      [ ..     slice_ref     [ptr = 0xef, len = 3]  ]   -->     [ 0xef  'h'     ]  }
//                                                                [ ..    'e'     ]  }  <<-- *slice_ref = "hel"
//                                                                [ ..    'l'     ]  }
//                                                                [ ..    'l'     ]
//                                                                [ ..    '0'     ]
//                   STACK                                    SOME MEMORY (stack or heap)
// By also including the slice length in the reference, this lets us talk about part of a data and give our reference a known size to refer to, while allowing the size of the slice reference type itself to be known.


// --------------------------------------------------------------------------------
// ## STRING SLICES (&str)
// A string slice (str) is essentially the type [u8], a sequence of UTF-8 encoded characters.
//  1. The type `str` has an unknown size at compile-time because it is designed to represent strings of arbitrary length.
//  2. Every variable in Rust must have a known size at compile time. This is true even for variables that refer to data
//     on the heap, because those variables' values are actually represented by pointers which have a known size.
//  Hence it's not possible to generically allocate a value of type `str` because its size is not fixed.
// A string slice reference (&str) provides a way to refer to the actual slice (string data) without needing a specific size.
fn string_slices(){
  // Create an owner of type String pointing to data "hello" allocated on the heap
  let s = String::from("hello");

  let slice = &s[0..2];
  let slice = &s[..2];
}
// ### STRING SLICES AS PARAMETERS
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
// ### STRING LITERALS AS SLICES
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
// The rules for mutable slice references are the same as for references.
//  ~ While a mutable reference is in scope, no new references can be declared, and no existing references (inc. the owner) can be used.
// This is true even for two mutable slice references that refer to different parts of memory but the same owner variable.
fn mutable_slices() {
  // Creates a fixed-size array xs stored on stack.
  let mut xs:  [i32; 5] = [1, 2, 3, 4, 5];

  // Creates a first mutable reference r1 to a slice that borrows part of the array xs.
  let r1: &mut [i32] = &mut xs[1 .. 4];
  println!("First element of the slice: {}", r1[0]);
  // r1 is now out of scope

  // Creates a second mutable reference r2 to a slice that borrows the remained of the array xs.
  let r2 = &mut xs[5 .. ];
  println!("First element of the slice: {}", r2[0]);
  // println!("First element of the slice: {}", r1[0]);    <<-- Not allowed, as this would mean r2 was used in the scope of r1.
  // r2 is now out of scope

  xs = [1,3,4,4,32];    // <<-- Allowed, as r1 and r2 are out of scope.
}
