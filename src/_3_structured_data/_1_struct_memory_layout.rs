// -----------------------------------------------
// # STRUCTS: MEMORY LAYOUT
// A STRUCT is represented in memory as a contiguous sequence of its field values whose types, in turn, determine their own memory representation.
//   Fields that can be allocated on the stack are done so like normal, represented on the stack directly by their data.
//   Fields that must be allocated on the heap are done so like normal, represented on the stack as references to that data on the heap.
struct User {
  active: bool,              // A stack-allocated type  consisting of a single byte (1 byte).
  sign_in_count: u64,        // A stack-allocated type  consisting of an unsigned 64-bit integer (8 bytes).
  username: String,          // A heap-allocated type consisting of { ptr (8 bytes), length (8 bytes), capacity (8 bytes) }
  email: String,             // A heap-allocated type consisting of { ptr (8 bytes), length (8 bytes), capacity (8 bytes) }
}

fn struct_repr(){
  let user1 = User {
    active: true,            // Underlying data stored on stack
    sign_in_count: 1,        // Underlying data stored on stack
    username: String::from("someusername123"),  // Underlying data stored on heap
    email: String::from("someone@example.com"), // Underlying data stored on heap
  };
}
// Stack (user1):
// +--------------------------------------------------+ 0x7ffdf000
// | active         : true                            | 1 byte
// +--------------------------------------------------+ 0x7ffdf001
// | padding        : 0x00 00 00 00 00 00             | 7 bytes (padding for alignment)
// +--------------------------------------------------+ 0x7ffdf008
// | sign_in_count  : 1                               | 8 bytes
// +--------------------------------------------------+ 0x7ffdf010
// | username.ptr   : 0x60001234                      | 8 bytes
// | username.len   : 17                              | 8 bytes
// | username.cap   : 17                              | 8 bytes
// +--------------------------------------------------+ 0x7ffdf028
// | email.ptr      : 0x60002234                      | 8 bytes
// | email.len      : 20                              | 8 bytes
// | email.cap      : 20                              | 8 bytes
// +--------------------------------------------------+ 0x7ffdf040
// Heap:
// +--------------------------------------------------+ 0x60001234
// | username       : "someusername123"               | 17 bytes + 1 null terminator
// +--------------------------------------------------+
// + ...                                              |
// +--------------------------------------------------+ 0x60002234
// | email          : "someone@example.com"           | 20 bytes + 1 null terminator
// +--------------------------------------------------+

// ## USING STRUCTS
fn structs_usage(){
  // [Mutability]
  //    Mutability is always inherited in Rust. There is no intrinsic notion of mutability in the definitions of datatypes.
  //    Hence, there is no intrinsic notion of mutability in struct definitions so their fields cannot be marked `mut`.
  //    Instead, fields always inherit the mutability of the containing structs which is decided by the owner that creates that the data.

  // Creating a mutable struct instance
  let mut user1 = User {
    active: true,  sign_in_count: 1,
    username: String::from("someusername123"), email: String::from("someone@example.com"),
  };
  // Accessing a field
  let b = user1.active;
  // Mutating a field
  user1.active = false;

  // Syntax sugar: assigning fields with function arguments:
  fn build_user(email_: String, username: String) -> User {
      User {
          active: true,  sign_in_count: 1,
          email: email_,    // we can explicitly assign the argument to the field
          username,         // or implicitly assign it if the argument matches the field name
      }
  }
  // Syntax sugar: reusing only parts of structs.
  let user2: User = User {
      username: String::from("anotherexample"),
      ..
      user1
  };
}
