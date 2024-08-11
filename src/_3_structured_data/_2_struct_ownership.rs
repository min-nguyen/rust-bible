// -----------------------------------------------
// # STRUCT OWNERSHIP TRANSFER: MOVE, COPY, and CLONE for Structs
//
// When assigning a struct from one variable to another, the rules for ownership transfer are the same as for any type.
// 1. By default, a MOVE occurs unless the struct implements the Copy trait.
//    The struct stored in the original variable is moved to the new variable.
//    The original variable is invalidated, and the new variable is now the owner of the struct.
// 2. A COPY happens if the assigned struct implements the Copy trait. This only applies to structs whose fields don't involve heap allocation.
//    The struct stored in the original variable is copied to the new variable.
//    Both the original and new variables remain valid, and they each own independent copies of the same struct.
// 3. A CLONE happens if the assigned struct is a result of clone(). This only applies to values that involve heap allocation.
//    The struct stored in the original variable is copied to the new variable, but any contained pointers
//    now point to a newly allocated region on the heap.
//    On the heap, the data that the struct pointed to is copied to the new region.
//    Both the original and new variables remain valid, and they each own independent copies of the same data on the heap.
//
// -------------------------------------------------------------------------------------------------
// ## An important note about Ownership Transfer in practice.
//
// Ownership transfer is an abstract concept, and it is not usually productive to think of how it happens in memory.
// That is, transferring ownership doesn't necessarily do anything in memory at all.
// Semantically, all MOVE, COPY, and CLONE perform a memcpy i.e. an actual copy in memory.
// In practice, a memcpy won't happen unless necessary, and the optimiser can do anything as long as it does not change the behaviour of your program.
//
//  - A MOVE may or may not copy the same value on the stack.
      // - For example, below COULD produce two copies of USER on the stack.
                        // struct User {
                        //   active: bool,
                        //   sign_in_count: u64,
                        // }
                        // fn structs_ownership(){
                        //   let user1 = User {
                        //     active: true,
                        //     sign_in_count: 1,
                        //   };
                        //   let user2 = user1;
                        // }
/// - A COPY may or may not copy the same value on the stack.
       // - For example, below COULD reduce to one user on the stack, and both `user2` and `user1` refer to the same address on the stack
                        // #[derive(Clone, Copy)]
                        // struct User {
                        //   active: bool,
                        //   sign_in_count: u64,
                        // }
                        // fn structs_ownership(){
                        //   let user1 = User {
                        //     active: true,
                        //     sign_in_count: 1,
                        //   };
                        //   let user2 = user1;
                        // }
/// - A CLONE may or may not copy the same value on the heap, and may or may not copy the same value on the heap.
///
// Having a mental model where every copy is a new value stored is fine, as long as you don't use this mental model to reason about performance.

struct User {
  active: bool,              // A stack-allocated type  consisting of a single byte (1 byte).
  sign_in_count: u64,        // A stack-allocated type  consisting of an unsigned 64-bit integer (8 bytes).
}
fn structs_ownership(){
  let user1 = User {
    active: true,
    sign_in_count: 1,
  };

  let user2 = user1;
}