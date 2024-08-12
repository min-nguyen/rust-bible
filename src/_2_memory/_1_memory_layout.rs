// -----------------------------------------------
// # OVERVIEW: Binary, Stack, Heap, and Static Data Segment
// https://github.com/amindWalker/Rust-Layout-and-Types
// https://samolusola.me/understanding-stack-and-heap-memory-in-rust

// ## Kernel Virtual Memory Space
//
// The Virtual Memory Space, reserved for the OS kernel, allows the OS to present physical memory to programs as a large and contiguous block of memory.
// The OS keeps track of the memory pages that are currently being used by the kernel and those that are available for use by programs.
// It also maps physical memory addresses to virtual memory addresses to access hardware devices and other system resources.

// A program is typically provided the following virtual memory model.
// (Memory addresses increase in bytes, e.g. 0x7ffeefbff4a0 + 1 byte = 0x7ffeefbff4a1.)
// +-------------------------------+
// |                               |
// |           Stack               |   <-- Managed by OS at runtime
// |             ↓                 |
// +-------------------------------+
// |                               |
// |                               |
// +-------------------------------+
// |             ↑                 |
// |           Heap                |   <-- Managed by OS at runtime
// |                               |
// |                               |
// +-------------------------------+  \
// |            BSS                |   |
// | (Uninitialized Static Data.)  |   |
// +-------------------------------+   |
// |            DATA               |   |- Included in Executable Binary
// |   (Initialized Static Data.)  |   |
// +-------------------------------+   |
// |        Text Segment           |   |
// |       (Program Code)          |   |
// +-------------------------------+  /
// |       Environment Vars        |  <-- Managed by OS at runtime
// +-------------------------------+
// |      Command Line Args        |  <-- Managed by OS at runtime
// +-------------------------------+

// -------------------------------------------------------------------------------------------------
// ## Binary
//
//   The output of the compilation process is a binary executable, which contains machine code and sections for different types of data.
//   It includes instructions for setting up and managing the stack and heap.
//   Executing the binary will have the OS load it into memory and begin executing the instructions.
//   Parts of the Binary include:
//   - Text Segment (Program Code)
//     This is where the Rust code is compiled (by LLVM) into machine code and stored for later execution.
//     The actual execution of the machine code instructions typically occurs elsewhere in memory.
//   - Data Segment (Initialised Static Data)
//     This is a special read-only region of memory for initialised static variables (which can be treated
//     with the same lifetime as the program and not bound to a specific scope) which have a defined value at compile-time
//     that does not change at run time.
//   - BSS (Uninitialised Static Data)
//     This stores uninitialised static variables.


// -------------------------------------------------------------------------------------------------
// ### Stack
//
//    The Stack is a fixed size region of memory that can store values with a known size (such as pointers) at compile-time.
//    It starts from a higher address and grows downwards towards lower addresses.
//    It stores local variables, functions arguments, and return addresses.

//    The binary’s instructions manage a stack pointer and allocate "stack frames" for function calls.
//       1. Every function call uses a new stack frame that provides enough memory for its arguments, local variables, and return address.
//       2. As variables are introduced, their data is pushed onto the stack within that function's stack frame.
//       3. Every function exit pops all the data in the stack frame off the stack.

// Example:
fn _main() {
    let x = 48;
    let y = &x;
    let z = _double(y);
    println!("{z}");
}
fn _double(n: &i32) -> i32 {
    n * 2
}
    // An Informal Mental Model of what *COULD* happen (specific details are imprecise but harmless for understanding.):
    // 1. The main() function is called.
    //    A stack frame is used for the main() function.
    //    The stack pointer is updated to point to the new stack frame.
    //    The local variable x is pushed to the stack, and stores 48.
    //    The local variable y is pushed to the stack, and stores a reference to x.
    // +----------------------------------------+
    // | Stack Frame: _main()                   |
    // +----------------------------------------+ 0x7ffeefbff4a0
    // | x: 48                                  |  <--- `x` stores the value `48`.
    // +----------------------------------------+ 0x7ffeefbff4a4
    // | y: 0x7ffeefbff4a0                      |  <--- `y` is a reference to `x``.
    // +----------------------------------------+
    // 2. The function double() is called, and the return address is stored in the stack.
    //    A new stack frame is used for the double() function.
    //    The stack pointer is updated to point to the new stack frame.
    //    The argument n is is pushed to the stack, and stores a reference to x.
    // +----------------------------------------+
    // | Stack Frame: _main()                   |
    // +----------------------------------------+ 0x7ffeefbff4a0
    // | x: 48                                  |  <--- `x` stores the value `48`.
    // +----------------------------------------+ 0x7ffeefbff4a4
    // | y: 0x7ffeefbff4a0                      |  <--- `y` is a reference to `x`.
    // +----------------------------------------+ 0x7ffeefbff4a8
    // | Return Address: _main()                |  <--- The return address in the text segment for when `_double` completes.
    // +----------------------------------------+
    // | Stack Frame: _double()                 |
    // +----------------------------------------+ 0x7ffeefbff4b0
    // | n: 0x7ffeefbff4a0                      |  <--- `n` is a reference to `x`.
    // +----------------------------------------+
    // 3. The double() function terminates.
    //    The stack frame is deallocated for the double() function.
    //    The stack pointer is updated to point to the previous stack frame.
    //    The local variable z is pushed to the stack, and stores the return value `48*2`.
    // +----------------------------------------+
    // | Stack Frame: _main()                   |
    // +----------------------------------------+ 0x7ffeefbff4a0
    // | x: 48                                  |  <--- `x` stores the value `48`.
    // +----------------------------------------+ 0x7ffeefbff4a4
    // | y: 0x7ffeefbff4a0                      |  <--- `y` is a reference to `x`.
    // +----------------------------------------+ 0x7ffeefbff4a8
    // | z: 96                                  |  <--- `z` stores the result of `_double(y)`, which is `96`.
    // +----------------------------------------+
    // 4. The main() function ends and the whole program terminates.

// -------------------------------------------------------------------------------------------------
// ### Heap
//     The Heap is a flexibly sized region of memory that can change at runtime and stores dynamically sized data.
//     The Heap memory starts from lower addreses and grows upwards towards higher addresses.

//     The binary's instructions include calls to the allocator to request and free memory.
//      1. Allocation:
//         The allocator finds an empty spot in the Heap big enough for the space requested,
//         marks the spot as 'being in use', and returns a pointer to the memory address of that location.
//      2. Deallocation:
//         The allocator releases the allocated memory, marks the spot as available for future #
//         reallocation, and the pointer referencing that space becomes invalid.


// -------------------------------------------------------------------------------------------------
// ## Mental Model: STACK vs HEAP: What is it?
//    https://conradludgate.com/posts/stack_heap
//
// In a typical application process, your application will request memory, and the operating system will provide it.
//
// The stack is a linear region of memory that the OS will start a process with. As the name suggests, all memory is allocated in stack order. This means that the most recently allocated value is the first value to be deallocated.
//
// It's not unreasonable to want some values to live longer. To support that we must allocate them elsewhere. The "heap" is for these allocations and they can be deallocated with no required order.

// -------------------------------------------------------------------------------------------------
// ## Mental Model: STACK vs HEAP: Does it matter?
//    https://conradludgate.com/posts/stack_heap
//
// Stack vs heap is a tempting but wrong model when thinking about Rust types.
//
// Values can be stored anywhere.
//     You cannot tell whether any type will be stored on the stack, heap or binary.
//
//     For example, it's possible for local variables to go on the heap
//
// The mental model we should have is:
//     * All variables as simply storing a value in "memory".
// And in addition, we need to think about:
//     1. Whether the variable in fact points to another value
//        i.e. Does it involve indirection?
//     2. Whether that underlying value is owned by it
//        i.e. Does it manage a value? (usually meaning it is on the heap but not necessarily).
//
// The key takeaway is:
//    * In general, we shouldn't really worry about where things are stored since it barely makes a difference most of the time.
//
//      For example, we almost never care about whether something is on the stack. It can be a fact from which you draw further conclusions about what you can't do with it, but the stack does not have benefits that you can use explicitly in your program. The stack is cheaper than the heap allocator, that's all; it gets you performance, not functionality. Additionally, you can store as much data on the heap as you want, with as much indirection as you want. But if you don't have something on the stack pointing to it, you've lost it (unless its a static variable, but we'll ignore that here.)
//
// Hence, for the question:
//       "Is a &str a pointer to the stack or the heap"
// The answer is:
//       "It is a pointer to wherever it was allocated".
//
//
// -------------------------------------------------------------------------------------------------
// ## Access: Stack vs Heap
//
// Stack access is faster because we never have to follow a pointer to get there, it is always relative to the top of the stack. This is true even if the data isn't always at the top:
    // 1. Local variables in a function have fixed offsets from the stack pointer or base pointer, making access straightforward and fast.
    // 2. The stack's contiguous memory layout means that when the CPU loads data from the stack into its cache, it often loads adjacent data as well,
    // 3. Simple Pointer Arithmetic: Accessing stack variables involves simple pointer arithmetic, which is efficient and fast.
// Heap access is slower because you have to follow a pointer to get there: contemporary processors are faster if they jump around less in memory.

// -------------------------------------------------------------------------------------------------
// ## Allocation: Stack vs Heap
//
//   Stack pushing is faster because the allocator never has to search for a place to store new data; that location is always at the current stack pointer, which is trivially maintained (by simple pointer arithmetic).
//   Heap allocation requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
