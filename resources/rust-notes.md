# Installing rustup on Linux

Install:
```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
Restart shell, then:
$ rustc --version
Updating:
$ rustup update
```

# Cargo

## Creating a Cargo project:

```sh
$ cargo new hello_cargo
```

You’ll see that Cargo has generated two files and one directory for us: a Cargo.toml file and a `src` directory with a `main.rs` file inside.
1. This Cargo.toml is Cargo’s configuration format.
  - [package], is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.
  - The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use.
  - The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates.
2. Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.

## Structure of a Cargo Project

## Building & Running a Rust Project

By default, this command creates an executable file for `main.rs` in target/debug/hello_cargo rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug.

```sh
$ cargo build
$ ./target/debug/hello_cargo
```

Alternatively, we can build and then execute in one command:
```sh
$ cargo run
```

And we can also check if a Cargo project compiles without producing an executable:

```sh
$ cargo check
```

If there are multiple executables i.e. binaries in the cargo.toml file, specified under `[[bin]]` with each rust file having its own main() function:


We can run specific ones by using `--bin <binary_name>` to select the binary.

```sh
$ cargo run --bin main1
```


# Variables and references

## (Immutable) Variables

Variables are declared using `let .. = ..;`, and are immutable by default and so cannot be reassigned values:
```rust
let x : i32 = 5; // x is immutable
x = 6;           // not allowed
```

  (The `let` clause shadows previous declared variables of the same name.
  ```rust
  let x : i32 = 5;
  let x : i32 = 6  // shadows previous x, and is allowed
  ```
  )

### Constants

Constants are declared using `const .. = ..;` and exactly the same as immutable variables, except:
  1. They can be declared in any scope including the global scope
  2. They can only be set to constant expressions and not the result of a value that is only computable at runtime.
```rust
const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
```

### Mutable variables

Variables can be declared mutable, allowing them to be reassigned values:
```rust
let mut x : i32 = 5;  // x is mutable
x = 6;                //  allowed
```

## (Immutable) References

A reference is the address of a variable's value, and is by default immutable.
Any variable can have a reference created to it by using &.
```rust
let x : i32 = 5; // x is immutable

foo(&x);           // create a reference to x and pass it to foo;
// or
let y : &i32 = &x; // create a reference to x and assign it to a new variable y;
foo(y);
```

### Mutable references
A mutable reference allows you to mutate the value at the address, not the address itself.
Any mutable variable can have a mutable reference created to it by using &mut.

```rust
let mut x : i32 = 5; // x is mutable

foo(&mut x);               // create a mutable reference to x and pass it to foo;
// or
let y : &mut i32 = &mut x; // create a mutable reference to x and assign it to a new variable y;
foo(y);
```

As variables can be references, we can also have combinations of (im)mutable variables that are (im)mutable references.
  - `y: &i32`: Immutable variable y is an immutable reference to an i32 value.
    You're not allowed to change anything.
  - `mut y: &i32`: Mutable variable y is an immutable reference to an i32 value.
    You're allowed to point y at a new memory location but not to change the contents of the memory it's pointing at.
  - `y: &mut i32`: Immutable variable y is a mutable reference to an i32 value.
    You're allowed to modify the contents of the memory y is pointing at, but not to change where it's pointing.
  - `mut y: &mut i32`: Mutable variable y is a mutable reference to an i32 value.
    You're allowed to modify the memory y is pointing at or to point y at a new location.

# Data Types

Every value in Rust is of a certain data type, where type annotations are specified with `:`.

There are two data type subsets: scalar and compound.

## Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floats, Booleans, and characters.

There are also strings. Allocating a string is not something that

## Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Tuples
A tuple groups values with a variety of types into one compound type.
We create a tuple by writing a comma-separated list of values inside parentheses, `(a, b, ...)`.
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```
We can pattern match to destructure a tuple value:
```rust
let (x, y, z) = tup;
```
We can access tuple elements directly by using `(.)` followed by an index:
```rust
let y = tup.2;
```

### Arrays
An array groups values with the same type.
We create an array by writing a comma-separated list of values inside square brackets, `[a, b ...]`.
```rust

```

# Ownership

The most primitive string type is `str` called a string slice, usually seen in its borrowed form `&str`. String literals are string slices, and have a static lifetime meaning they are guaranteed to be valid for the duration of the entire program.
```rust
let s: str = "hello";
```
A `&str` has two components: a pointer to some bytes and a length.

- every variable in Rust must have a known size at compile time
- even for values whose size is only dynamically known, to access those values, we can only do so through pointers, which have a known size. i.e. variables are either fixed sized values, like u32, or pointers, which also have a fixed size.
- for the compiler to know the size of the string literal "hello" in the first place, it has to be stored somewhere in the first place. The only way you can concretely represent a slice in memory is through its address and its size.


## Slices


### Strings

 Strings are for mutable, growable strings and are stored on the heap.

 `str` and `&str` : String Slices and References to String Slices

 String Slices (`str`) for immutable strings is called a string slice, and is used to represent string literals.
 However, the references to string slices that we actually use (`&str`) are stored on the stack.
  ```rust
  let s: &str = "hello";
  ```

  The **string slice** `str` type represents a sequence of UTF-8 encoded characters in Rust.
  1. **The type `str` has an unknown size at compile-time** because it is designed to represent strings of arbitrary length.
      This is even though we know the size of a string literal, because the type `str` is not determined solely by the length of a single literal.
  2. **Every variable in Rust must have a known size at compile time**
      Even for variables whose values have a size that is only dynamically known (stored on the heap),
      to access those values, we can only do so through pointers which have a known size.
      Thus, being able to allocate a `str` at compile-time would imply that all values of type `str` have the same known siz, which isn't true.
  3. **Hence it's not possible to generically allocate a value of type `str`** because its size is not fixed in the way primitive types like `i32` or `f64` are.

  The **string slice reference** `&str` type provides a way to refer to the actual slice (string data) without needing a specific size.
    - The **only way you can concretely represent a slice in memory** is through a **fat pointer** its address and its size.
      Hence `&str` has two components: a pointer to the start of the slice and a length being the number of characters in the slice.
      While values of type `str` can take on many sizes, values of type `&str` have a known fixed size.
  Then,  **string literal data itself** of type `str` is then stored in the read-only section of the binary.
    - Although that string data is represented using a dynamically sized type, its size is known at compile-time.

 String Literals: both String Slice References AND  String Slices
  As A String Slice Reference:
    - When declaring a string literal, Rust interprets it immediately as a reference to a string slice.
    - The type of "hello" is &str, meaning Rust understands it is actually a reference to a string slice of type.
    - The statement`let x: &str = "hello"` assigns this reference to the variable x, meaning we are borrowing the string slice.
    - This is in contrast to `let x: &u32 = 5`, because 5 is a `u32` value, not a reference to a u32.
  As The String Slice Itself:
    - The value at a string slice reference is the actual string data it points to.
    - The value at the reference "hello" is the sequence of characters 'h', 'e', 'l', 'l', 'o'.


# Enums and pattern matching

A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.