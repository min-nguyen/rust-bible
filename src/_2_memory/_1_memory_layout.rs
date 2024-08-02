// -----------------------------------------------
// # OVERVIEW: Binary, Stack, and Heap Relationship

// -------------------------------------------------------------------------------------------------
// ### Binary:
//     The output of the compilation process is a binary executable.
//     The binary contains machine code and sections for different types of data, including instructions for managing the stack and heap.
//     When you execute the binary, the operating system loads it into memory and starts executing its instructions, which includes setup for stack and heap usage.

// -------------------------------------------------------------------------------------------------
// ### Stack
//     The stack can store values that have a known, fixed size (such as pointers) at compile-time.
//     The binary’s instructions manage the stack for function calls and local variable storage, and manipulate the stack pointer to allocate and deallocate memory as functions are called and return.
//     1. Every function call pre-allocates a stack frame, providing enough memory for its arguments and local variables.
//     2. As local variables are introduced, their data is pushed onto the stack.
//     3. Every function exit pops all the data in the stack frame off the stack.
fn foo( x : u32     // <-- x is pushed onto stack
      , y : &u32) { // <-- y is pushed onto stack
    let z = x;         // <-- z is pushed onto stack
}                      // <-- z, y, then x are popped off of stack
//    +--------+
//    |        |
//    +--------+  \
//    | z      |  |
//    +--------+  |
//    | y      |  |
//    +--------+  |- pre-allocated stack memory for foo() at compile-time
//    | x      |  |
//    +--------+ /

// -------------------------------------------------------------------------------------------------
// ### Heap
//     The heap can store data for a specific requested amount of space, which may be unknown at compile time and may change at runtime.
//     The binary instructions manage heap allocations for dynamically allocated memory, including calls to the allocator to request and free memory.


// -------------------------------------------------------------------------------------------------
// ### Access: Stack vs Heap:
// Stack access is faster because we never have to follow a pointer to get there, it is always relative to the top of the stack. This is true even if the data isn't always at the top:
    // 1. Local variables in a function have fixed offsets from the stack pointer or base pointer, making access straightforward and fast.
    // 2. The stack's contiguous memory layout means that when the CPU loads data from the stack into its cache, it often loads adjacent data as well,
    // 3. Simple Pointer Arithmetic: Accessing stack variables involves simple pointer arithmetic, which is efficient and fast.
// Heap access is slower because you have to follow a pointer to get there: contemporary processors are faster if they jump around less in memory.


// -------------------------------------------------------------------------------------------------
// ### Allocation: Stack vs Heap
//   Stack pushing is faster because the allocator never has to search for a place to store new data; that location is always at the current stack pointer, which is trivially maintained (by simple pointer arithmetic).
//   Heap allocation requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
