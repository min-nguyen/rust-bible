// -----------------------------------------------
// # STRUCTS: MEMORY LAYOUT
//
// A struct is represented in memory as a contiguous sequence of its field values.
// The type of each field determines its own memory representation.
//
//    As noted before [_1_memory_layout.rs], stack vs heap is a tempting but wrong model when thinking about Rust types.
//    That is, values can be stored anywhere and it is not easy to tell whether a type will be stored on the stack, heap or binary.
//
struct User {
  active: bool,              // active is a bool (1 byte), likely on the stack
  sign_in_count: u64,        // sign_in_count is an integer (8 bytes), likely on the stack
  username: String,          // A heap-allocated type consisting of { ptr (8 bytes), length (8 bytes), capacity (8 bytes) }
  email: String,             // A heap-allocated type consisting of { ptr (8 bytes), length (8 bytes), capacity (8 bytes) }
}

    // Stack:
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
  //    Instead, fields always inherit the mutability of the containing structs which is decided by the owner of that data.

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
