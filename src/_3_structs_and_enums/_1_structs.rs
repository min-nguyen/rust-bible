// -----------------------------------------------
// # STRUCTS
//
// A struct represents a product of values, all contained inside a single constructor.
// It is represented in memory as a contiguous sequence of its field values.
// The type of each field determines its own memory representation.
//
//    struct StructName { field_name: field_type
//                      , ...}
//
//    let x = StructName { field_name: field_value
//                      , ...}
//
//    As noted before [_1_memory_layout.rs], stack vs heap is a tempting but wrong model when thinking about Rust types.
//    That is, values can be stored anywhere and it is not easy to tell whether a type will be stored on the stack, heap or binary.

// -----------------------------------------------
// ## Defining Structs
struct User {
  active: bool,              // active is a bool value
  sign_in_count: u64,        // sign_in_count is an int value
  username: String,          // username is a { ptr, len, cap } value that manages a string on the heap
}

    // Informal Mental Model: what *COULD* happen:
    // Stack:
    // +--------------------------------------------------+ 0x7ffdf000
    // | active         : true                            | 1 byte
    // +--------------------------------------------------+ 0x7ffdf001
    // | padding        : 0x00 00 00 00 00 00             | 7 bytes (padding for alignment)
    // +--------------------------------------------------+ 0x7ffdf008
    // | sign_in_count  : 1                               | 8 bytes
    // +--------------------------------------------------+ 0x7ffdf010
    // | username {ptr: 0x60001234, len: 17, cap: 17}     | 24 bytes
    // +--------------------------------------------------+ 0x7ffdf028
    // Heap:
    // +--------------------------------------------------+ 0x60001234
    // | username       : "someusername123"               | 17 bytes + 1 null terminator
    // +--------------------------------------------------+

// -----------------------------------------------
// ## Using Structs
fn using_structs(){
  // [Specify struct values]
  let mut user1 = User {
    active: true,
    sign_in_count: 1,
    username: String::from("someusername123"),
  };
  // [Accessing a field]
  let b = user1.active;
  // [Mutating a field]
  user1.active = false;

  // [Syntax sugar: assigning fields with function arguments.]
  // We can implicitly assign a function argument to a field if it matches the field name
  fn build_user(username: String) -> User {
      User {
          active: true,
          sign_in_count: 1,
          username,
      }
  }

  // [Syntax sugar: reusing only parts of structs.]
  // We can specify new field values where desired, and then use `..` followed by an existing struct to reuse its field values
  let user2: User = User {
      username: String::from("anotherexample"),
      ..
      user1
  };
}

// -----------------------------------------------
// ## Mutability in Datatypes: Structs
//
//    Mutability is always inherited in Rust, and there is no intrinsic notion of mutability in the definitions of datatypes.
//    Hence structs fields always inherit the mutability of the struct value, which is decided by the owner of that data, and
//    cannot be defined as inherently mutable.

fn mutability_in_structs(){
  // immut_user is an immutable struct, and so all its fields are immutable
  let immut_user: User = User {
    active: true,
    sign_in_count: 1,
    username: String::from("someusername123"),
  };
  // hence its fields can only be read
  let b = immut_user.active;

  // mut_user now *mutably* owns the struct data moved from immut_user, and so all its fields are mutable
  let mut mut_user: User = immut_user;
  // hence its fields can now be mutated
  mut_user.active = false;
}


// -----------------------------------------------
// ## Ownership Transfer for Structs: Full and Partial Moves
//
// The rules of ownership transfer are the same for structs as for any value or datatype containing values.
//
// When assigning a **struct** from one variable to another, a full Move, Copy, or Clone applies for the struct type.
//    let struct2 = struct1;
// A Move, Copy, or Clone happens for all its fields, and determines whether or not struct1 is still valid.
//
// When assigning a **struct field** of a variable to another, a partial Move, Copy, or Clone applies for the struct according
// to the specific field type.
//    let x = struct1.field;
// A Move, Copy, or Clone applies to just that field, and determines whether or not struct1.field is still
// valid, which then determines if struct1 is **fully valid** or only **partially valid**.

// -------------------------------------------------------------------------------------------------
// [MOVE]
struct UserMove {
  active: bool,              // copyable
  sign_in_count: u64,        // copyable
  username: String,          // only moveable/cloneable
  email: String,             // only moveable/cloneable
}

fn move_struct(){
  let mut moveable_user = UserMove {
    active: true,
    sign_in_count: 1,
    username: String::from("someusername123"),
    email: String::from("someusername123"),
  };


  // ---- Full move of moveable struct affects ownership.
  // moved_user('s value) is a User struct moved from moveable_user.
  // it trivially owns a bool and int, and also owns a string on the heap.
  let mut moved_user = moveable_user;  // <<-- user1 is no longer valid

  // Not allowed as moveable_user is no longer valid.
  // print!("{0}", moveable_user.active); // Error: moveable_user's value, and hence moveable_user.active, was moved

  // Not allowed as moveable_user is no longer valid.
  // let moved_user2 = moveable_user;        // Error: moveable_user's value was moved

  moveable_user = moved_user; //  <<-- reset moved_user


  // ---- Partial move of moveable struct  affects ownership.
  // moved_username('s value) owns a string on the heap moved from moveable_user.username
  let moved_username : String = moveable_user.username;

  // Not allowed as moveable_user.username is no longer valid
  // print!("{0}", moveable_user.username);   // Error: moveable_user.username was moved

  // Not allowed as moveable_user is only partially valid.
  // let mut moved_user2 = moveable_user;   // Error: moveable_user.username was moved

  // Allowed as moveable_user.email is still valid
  print!("{0}", moveable_user.email);

  moveable_user.username = moved_username; // <<-- reset moveable_user


  // ---- Partial copy of moveable struct does not affect ownership.
  // copied_active('s value) owns an int copied from moveable_user.active
  let copied_active = moveable_user.active;

  // Allowed as moveable_user.active is still valid.
  print!("{0}", moveable_user.active);

  // Allowed as moveable_user is still valid.
  let moved_user = moveable_user;

  moveable_user = moved_user; // <<-- reset moveable_user


  // ---- Partial clone of moveable struct does not affect ownership.
  // cloned_username('s value)  owns a string cloned from moveable_user.username
  let cloned_username = moveable_user.username.clone();

  // Allowed as moveable_user.active is still valid.
  print!("{0}", moveable_user.active);

  // Allowed as moveable_user is still valid.
  let copied_user2 = moveable_user;
}

// -------------------------------------------------------------------------------------------------
// [COPY]
#[derive(Clone, Copy)]
struct UserCopy {
  active: bool,              // copyable
  sign_in_count: u64,        // copyable
}
fn copy_struct(){
  let copyable_user = UserCopy {
    active: true,
    sign_in_count: 1,
  };

  // ---- Full copy of copyable struct does not affect ownership.:
  // copied_user('s value) is a User struct copied from copyable_user.
  // it trivially owns a bool and int
  let copied_user = copyable_user;

  // Allowed as copyable_user.active is still valid.
  print!("{0}", copyable_user.active);

  // Allowed as copyable_user is still valid.
  let copied_user2 = copyable_user;

  // ---- Partial copy of copyable structs does not affect ownership.
  //      and have the same rules as for moveable structs, and don't affect ownership.
  let copied_active = copyable_user.active;
}

// -------------------------------------------------------------------------------------------------
// [COPY]
#[derive(Clone)]
struct UserClone {
  active: bool,              // copyable
  sign_in_count: u64,        // copyable
  email: String,             // only moveable/cloneable
}
fn clone_struct(){
  let mut cloneable_user = UserClone {
    active: true,
    sign_in_count: 1,
    email: String::from("someusername123"),
  };

  // ---- Full clone of cloneable struct does not affect ownership.
  // cloned_user('s value) is a User struct cloned from cloneable_user.
  // it trivially owns a bool and int, and also owns a string on the heap cloned from cloneable_user.email
  let cloned_user = cloneable_user.clone();

  // Allowed as cloneable_user.active is still valid.
  print!("{0}", cloneable_user.active);

  // Allowed as cloneable_user is still valid.
  let moved_user = cloneable_user;

  let cloneable_user = moved_user; // reset

  // ---- Partial clone of cloneable structs does not affect ownership.
  //      and have the same rules as for moveable structs, and does not affect ownership.
  let cloned_email = cloneable_user.email.clone();
}