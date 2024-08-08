// -----------------------------------------------
// # STRUCTS: OWNERSHIP, COPYing, MOVEing, and CLONEing Structs
struct User {
  active: bool,              // A stack-allocated type  consisting of a single byte (1 byte).
  sign_in_count: u64,        // A stack-allocated type  consisting of an unsigned 64-bit integer (8 bytes).
  username: String,          // A heap-allocated type consisting of { ptr (8 bytes), length (8 bytes), capacity (8 bytes) }
}
