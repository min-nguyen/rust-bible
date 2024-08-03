// -----------------------------------------------
// # OVERVIEW: Binary, Stack, Heap, and Static Data Segment
// https://github.com/amindWalker/Rust-Layout-and-Types

// -------------------------------------------------------------------------------------------------
// Kernel Virtual Memory Space
// This is the segment of virtual memory reserved for the OS kernel. It allows the OS to present the computer's physical memory to applications as a large and contiguous block of memory. The OS keeps track of the memory pages that are currently being used by the kernel and those that are available for use by applications. It also maps physical memory addresses to virtual memory addresses to access hardware devices and other system resources.

// -------------------------------------------------------------------------------------------------
// Memory Address Range
//   The memory address range is bounded by the word size of the CPU.
//   In a 64-bits processor, the word size is 64 bits or 8 bytes.


// -------------------------------------------------------------------------------------------------
// Binary
//   The output of the compilation process is a binary executable, which contains machine code and sections for different types of data, including instructions for setting up and managing the stack and heap. Executing the binary will have the OS load it into memory and begin executing the instructions.
//   Parts of the Binary include:
//   - Text Segment (Code Segment)
//     This is where the Rust code is compiled (by LLVM) into machine code and stored for later execution.
//     The actual execution of the machine code instructions typically occurs elsewhere in memory.
//   - Data Segment
//     This is used to store initialised static variables, i.e. global and static local variables,
//     which have a defined value at compile-time that does not change at run time.
//   - BSS (Block Started by Symbol)
//     This stores uninitialised variables.


// In Rust, memory is organized into stack, heap, and a static memory region.

// -------------------------------------------------------------------------------------------------
// ### Stack
//    The Stack is a fixed size region of memory that can store values with a known size (such as pointers) at compile-time.
//    The Stack memory starts from a higher address and grows downwards towards lower addresses.

//    The binary’s instructions manage the stack by managing a stack pointer and allocating "stack frames" for function calls that can store their local variables, pushing and popping off this data in a FIFO manner:
//       1. Every function call pre-allocates a stack frame, providing enough memory for its arguments and local variables.
//       2. As local variables are introduced, their data is pushed onto the stack inside the region of that function's frame.
//       3. Every function exit pops all the data in the stack frame off the stack.

// Example:
fn _main() {
    let a = 48;
    let b = _double(a);
    println!("{b}");
}

fn _double(n: i32) -> i32 {
    n * 2
}
// 1. A stack frame is created for the main() function and the stack pointer is updated to point to the new stack frame. The local variable a is stored in the stack frame and takes up 4 bytes of memory.
// 2. When the variable b calls the function double(), a new stack frame is created for the double() function. The stack pointer is updated to point to the new stack frame, but the change in the stack pointer depends on the size of the function arguments and local variables.
// 3. The parameter n is stored in the stack frame for the double() function and takes up 4 bytes of memory. The return address is stored in the stack, and its size depends on the architecture of the system and the operating system.
// 4. The double() function terminates and the operating system deallocates the stack frame for the double() function. The stack pointer is updated to point to the previous stack frame, and the return value is stored in the variable b in the main() function. The main() function ends and the whole program terminates.

//    +----------------------------+  \
//    | _main ()                   |  |
//    +----------------------------+  |
//    | a = 48                     |  |
//    +----------------------------+  |
//    | b = 96                     |  |- STACK
//    +----------------------------+  |
//    | _double ()                 |  |
//    +----------------------------+  |
//    | n = 48                     |  |
//    +----------------------------+  |
//    | fn return addr 0x12f = 96  |  |
//    +----------------------------+  /
//    |                            |  \
//    | ...............            |   |- FREE MEMORY
//    |                            |  /
//    +----------------------------+  \
//    |                            |  |
//    | ............            |  |  |- HEAP
//    |                            |  |
//    +----------------------------+  /

// -------------------------------------------------------------------------------------------------
// ### Heap
//     The Heap is a flexibly sized region of memory that can change at runtime.
//     The Heap memory starts from lower addreses and grows upwards towards higher addresses.

//     The binary's instructions include calls to the allocator to request and free memory.


// -------------------------------------------------------------------------------------------------
// ### Static Memory Region
//     The static data segment is a special read-only region of memory that is part of the program's binary.
//     It stores "static variables" which can be treated with the same lifetime as the program and not bound to a specific scope.
//     These include global variables and static local variables.


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
