// -----------------------------------------------
// # STRUCTS: OWNERSHIP, COPYing, MOVEing, and CLONEing Structs
struct User {
  active: bool,              // A stack-allocated type  consisting of a single byte (1 byte).
  sign_in_count: u64,        // A stack-allocated type  consisting of an unsigned 64-bit integer (8 bytes).
  username: String,          // A heap-allocated type consisting of { ptr (8 bytes), length (8 bytes), capacity (8 bytes) }
}
// ## OWNERSHIP TRANSFER in STRUCTS: Copying, Moving, and Cloning Structs
// When transferring a struct from one variable to another:
// If a struct contains only stack-allocated types that implement the Copy trait, the struct itself can be copied rather than moved.
//   So the original variable remains valid.
// If a struct contains heap-allocated types, the entire struct including all its fields are moved.
//   So the original variable becomes invalid.

fn structs_ownership(){
  // user1 owns the
  let  user1 = User {
    active: true,
    sign_in_count: 1,
    username: String::from("someusername123")
  };

  //
  let  user2 = user1;
  print!("{0}", user1.active); // = false;
}