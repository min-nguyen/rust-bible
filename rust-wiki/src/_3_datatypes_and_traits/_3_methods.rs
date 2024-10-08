// -----------------------------------------------
// # METHODS (AND ASSOCIATED FUNCTIONS)
//
// An  `impl` block implements a set of "methods" and "associated functions" for a Struct or Enum type (or Trait not covered yet).
//
//    impl StructName/EnumName {
//      fn method_name(self : &Self, ...) -> ... {
//
//      }
//      fn assoc_fun_name(...) -> ... {
//
//      }
//    }
//
// The Rules of Impl Blocks:
//   - A struct or enum can have multiple `impl` blocks.
//   - The keyword `Self` is reserved to refer to the specific instance type.
//   - The keyword `self` is reserved for the first argument of each method, and refers to the specific instance.
//   - The method argument `self` without a type annotation is shorthand for `self : Self`.
//   - The method argument `&self` without a type annotation is shorthand for `self : &Self`.

// -----------------------------------------------
// ## Defining Methods (and Associated Functions)
//
// Syntax:
//
//    impl StructName/EnumName {
//      fn assoc_fun_name(...) -> ... {
//
//      }
//      fn method_name(&self, ...) -> ... {
//
//      }
//    }
//
// *Associated functions* have no `self` parameter.
// - These are often used as constructors that return a new instance of the type.
//
// *Methods* are a specific kind of associated function whose first parameter is always named `self`.
// - The type of the `self` variable can either be:
//   1. `Self`, and so `self` is a value that owns the struct instance moved or copied from the one the method is called on.
//            fn method_name(self, ....)  ... shorthand for ...  fn method_name(self: Self, ...)
//   2. `&Self`, and so `self`` is a immutable reference that borrows the struct instance the method is called on.
//            fn method_name(&self, ....) ... shorthand for ...  fn method_name(self: &Self, ...)
//   3. `&mut Self`, and so `self` is a mutable reference that borrows the struct instance the method is called on.
//            fn method_name(&mut self, ....) ... shorthand for ... fn method_name(self: &mut Self, ...)
// - The mutability of the `self` variable **itself** is determined when declaring it (here as a method argument) to own
//   a value, just like any other variable.
//   Although there is shorthand for expressing the types Self, &Self, and &mut Self, these is no shorthand for
//   expressing mutability of the variable `self`. Instead, we must explicitly write one of the following:
//            1.  fn method_name(mut self: Self, ...)
//            2.  fn method_name(mut self: &Self, ...)
//            3.  fn method_name(mut self: &mut Self, ...)
#[derive(Clone, Copy)]
struct Rectangle {        // Rectangle is copyable
  width: u32,
  height: u32,
}

impl Rectangle {
  // Rectangle::new is an associated function that constructs and returns a new Rectangle struct.
  fn new(width: u32, height: u32) -> Self {
      Rectangle{width, height}
  }
  // Rectangle::area_withselfval is a method that takes ownership of the receiver `self :: Rectangle`, which copies it
  fn area_withselfval(self) -> u32 {
      return self.width * self.height
  }
  // Rectangle::area_withselfref is a method that creates a reference that borrows the receiver `self :: Rectangle`
  fn area_withselfref(&self) -> u32 {
      return self.width * self.height
  }
}

#[derive(Clone)]
enum Message {
  Move{x: i32, y: i32},   //  Move has named fields for two  i32 values
  Write(String),          //  Write contains a single { ptr, len, cap } value that manages a string on the heap
}

impl Message {
  // Message::new is an associated function that constructs and returns a new Message::Move variant.
  fn new(x: i32, y: i32) -> Self {
      Message::Move{x, y}
  }
  // Message::sum_withselfval is a method that takes ownership of the receiver `self :: Message`, which moves it
  fn sum_withselfval(self) -> i32 {
      match self {
        // x own the i32 values x and y moved from self : Self
        Message::Move{x, y} => x + y,
        // s owns the String value moved from self : Self
        Message::Write(s) => 0,
      }
  }
  // Message::sum_withselfref is a method that creates a reference that borrows the receiver `self :: Message`
  fn sum_withselfref(&self) -> i32 {
      match self {
        // x_ref and y_ref are references to the i32 values x and y because self : &Self is a reference.
        Message::Move{x : x_ref, y: y_ref} => *x_ref + *y_ref,
        // s_ref is reference to the String value because self : &Self is a reference.
        Message::Write(s_ref) => 0,
      }
  }
}


// -----------------------------------------------
// ## Using Methods (and Associated Functions)
//
// Syntax:
//
//     let s : StructOrEnumType = ...;
//     s.method_name(...);
///
//     StructOrEnumType::fn_name(...);
//
// Using the method of a value (x : T) is written as `x.method()` without providing the self argument.
// Using the underlying method of a reference (x : &T) is written by either:
//     1. Explicitly dereference it, written as *x.method().
//     2. Implicitly dereference it, written directly as x.method(), whereby the Rust compiler automatically redeferences it.
//
// Using an associated function that is not a method is done by StructName::fun_name() or EnumName::fun_name();
//

fn using_methods_example_1(){
  // r is a new rectangle created by calling the Rectangle associated function new().
  let r: Rectangle = Rectangle::new(5, 5);
  // area1 is the result of calling r's method area_withselfval, which involves copying `r`.
  let area1: u32 = r.area_withselfval();
  // We can reuse r because it was never moved, only copied, so is still valid.
  // area2 is the result of calling r's method area_withselfref, which involves borrowing `r`.
  let area2: u32 = r.area_withselfref();
}


fn using_methods_example_2(){
  // m is a new message created by calling the Message associated function new().
  let m: Message = Message::new(5, 5);
  // x1 is the result of calling m's method sum_withselfval, which involves moving `m`.
  let x1: i32 = m.sum_withselfval();
  // We cannot reuse m because its ownership was moved when calling m.sum_withselfval();
  // let m2: Message = m; // Error: Use of moved value m.

  let m: Message = Message::new(5, 5);
  // x1 is the result of calling m's method sum_withselfref, which involves borrowing `m`.
  let x1: i32 = m.sum_withselfref();
  // We can reuse m because we only borrowed it as a reference when calling m.sum_withselfref();
  let m2: Message = m;
}
