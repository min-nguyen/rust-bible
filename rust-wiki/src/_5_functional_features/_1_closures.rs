// -----------------------------------------------
// # CLOSURES
//
// Regular function definitions (the syntax `fn``) are "static" in the sense they can cannot capture their
// dynamic environment, which includes local variables. They only capture their static environment, which
// includes other function definitions and global or static variables that are in scope.
// That is,
//    Static Environment: function definitions, global variables, and static variables that are in scope.
//    Regular functions can access these directly.
//
// Closures or lambdas (the syntax `| |`) are dynamic values and capture both the static and their dynamic environment.
// This hence lets us use the local variables from the scope they're defined within.
// That is,
//    Dynamic Environment: includes local variables that are in scope.
//    Closures can capture these, while regular functions cannot.
//
// Syntax:
//
//   | lambda_arg : T, ...| -> T { lambda_body }
//

fn closures_vs_fns(){
  // A local function definition (not a variable).
  // Although this is local to the current scope, it is included in the scope's static environment. (We *could* declare it at the end.)
  fn one_fn() -> u32{ 1 }
  // A local variable.
  let one_var: u32 = 1; // or we could even write: `one_var: fn() -> u32 = one_fn;`

  // A function def has access to other functions in scope (as these are included in its static environment).
  fn  add_one_fundef (y: u32) -> u32 { y + one_fn() }
  // A function def can't access local variables in scope (as these are only in its dynamic environment).
  // fn add_one_fundef (y: u32) -> u32 { y + one_var } ERROR!

  // A closure can access regular local variables in scope.
  let add_one_lamdef
      = |y: u32| -> u32 { y + one_var };
  let two: u32 = add_one_lamdef(1);
}

// -----------------------------------------------
// ## Closures: Moving or Borrowing the Captured Values into the Closure Body
//
// The **declaration** of a closure determines whether we move or borrow its captured value into the body.
//
// As closures are variables that own things, *declaring* one will capture values from their environment in
// three ways, which are the same as how a function can take a parameter:
//  1. Capturing Ownership: Moving (or Copying where possible) *all* the captured values it uses.
//       Syntax (using `move`)
//          let clsre = move |...| { body }
//  2. Capturing a Reference: Immutably Borrowing *all* the captured values it uses, which is the **default**.
//       Syntax (as normal):
//          let clsre = | ...| { body }
//  3. Capturing a Mutable Reference: Mutably Borrowing *all* the captured mutable values it uses.
//       Syntax (using `mut`):
//          let mut clsre = | ...| { body }
//
// The closure decides which of these to use based on what its body does with the captured values.
// (It is also possible to *selectively* move, copy, or borrow the captured values, by using some extra code.)

fn closure_move_in() {
  // x owns an i32 which implements Copy, and xs owns a Vec which does not.
  let x: i32 = 2;
  let xs: Vec<i32> = vec![1, 2, 3];

  // this closure, when declared, performs a Move of xs into the body, and a Copy of x.
  let fn_move_in = move || { print!("{x:?}, {xs:?}") }; // Permanently moves ys into the closure

  // can't use moved value: note that we didn't have to even call the function.
  // println!("{xs:?}"); // Error!
  // can use copied value
  println!("{x:?}");
}

fn closure_immutborrow_in() {
  // x owns an i32 which implements Copy, and xs owns a Vec which does not.
  let x: i32 = 2;
  let xs: Vec<i32> = vec![1, 2, 3];

  // this closure performs a (Immutable) Borrow of xs into the body (and a Copy of x).
  let fn_borrow_immut = || { print!("{x:?}, {xs:?}") };

  fn_borrow_immut();
  // can reuse the borrowed value after clsre_borrow has finished (immutably) using it
  println!("After calling closure: {xs:?}");
}

fn closure_mutborrow_in() {
  // x owns an i32 which implements Copy, and xs *mutably* owns a Vec which does not.
  let x: i32 = 2;
  let mut xs: Vec<i32> = vec![1, 2, 3];

  // this closure performs a Mutable Borrow of xs into the body (and a  Copy of x).
  let mut fn_borrow_mut = || { print!("{:?}", xs.push(x)) };

  fn_borrow_mut();
  // Can reuse the borrowed mutable value after clsre_borrow has finished using (and possibly mutating) it
  println!("After calling closure: {xs:?}");
}

// -----------------------------------------------
// ## Closures: Moving Captured Values out of the Closure Body
//
// The **calling** of a closure determines when we mutate or move its captured value back out of the body.
//
// A closure body that captures a value can do any of the following:
//  1. Move the captured value *out of* the closure, requiring us to first Move that captured value *into* the closure
//  2. Mutate the captured value, requiring us to first Mutably Borrow or Mutably Move that Mutable value *into* the closure.
//  3. Neither Move nor Mutate the value,


fn closure_move_out() {
  // x owns an i32 which implements Copy, and xs owns a Vec which does not.
  let x: i32 = 2;
  let xs: Vec<i32> = vec![1, 2, 3];

  // this closure Moves xs out of the body, meaning it already (implicitly) Moved xs into the body.
  let fn_move_out = || { return (xs, x) };

  // can't use moved value
  // println!("{xs:?}"); // Error!
  // can use copied value
  println!("{x:?}");
}


fn closure_mutate_out() {
  let mut xs: Vec<i32> = vec![1, 2, 3];

  // this closure mutates xs, where we Mutably Borrowed xs into the body.
  let mut fn_borrow_in_mutate =   || { xs.push(2) };

  fn_borrow_in_mutate();
  // we can reuse the borrowed value
  println!("{xs:?}"); // Error!

  // this closure mutates xs, where we Mutably Moved xs into the body.
  let mut fn_move_in_mutate = move  || { xs.push(2) };

  fn_move_in_mutate();
  // can't use moved value
  // println!("{xs:?}"); // Error!
}

// -----------------------------------------------
// ## Closures: The Fn Trait
//
// Closures automatically implement one, two, or all three of these Fn traits, depending on how their body handles its captured values:
//
//  1. FnOnce: Applies to all closures, as all can be called once.
//
//      A closure that moves captured values out of its body will only implement FnOnce, because it can only be called once.
//
//  2. FnMut: Applies to closures that don't move captured values out of their body, but may mutate them.
//
//      These closures can be called more than once while mutating their environment.
//
//  3. Fn: Applies to closures that don't move captured values out of their body, and that don't mutate them.
//
//      These closures can be called more than once without mutating their environment.
//


fn fn_trait() {

  //// 1. FnOnce
  // this closure when called Moves xs out of the body, meaning that its declaration has (implicitly) Moves xs into the body
  let  xs: Vec<i32> = vec![1, 2, 3];
  let fn_once_ = || { return (xs) }; // Moves xs into the closure, then when called, moves it back out.

  fn_once_();
  // fn_once(); // Error: use of moved value `xs` after it has already been moved

  //// 2. FnMut
  // this closure when called doesn't Move ys out of the body but mutates it, so implements FnMut
  let mut ys: Vec<i32> = vec![1, 2, 3];
  let mut fn_mut_ = move || { ys.push(2) };  // Permanently Moves ys into the closure,

  fn_mut_();
  fn_mut_(); // (we have only performed a single Move of ys when declaring the closure, so we can call this many times)

  //// 3. Fn
  // this closure when called neither moves or mutates ys out of the body, so implements Fn
  let mut ys: Vec<i32> = vec![1, 2, 3];
  let mut fn_ = move || { print!("{ys:?}") }; // Permanently Moves ys into the closure

  fn_();
  fn_(); // (we have only performed a single Move of ys when declaring the closure, so we can call this many times)

}

