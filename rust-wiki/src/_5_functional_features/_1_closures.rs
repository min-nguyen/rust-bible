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
// ## Closures: Moving, Copying, or Borrowing the Captured Values
//
// As closures are variables that own things, they can capture values from their environment in
// three ways, which are the same as how a function can take a parameter:
//  1. Capturing Ownership: Performing a Move (or Copy where possible) of all the captured values it uses.
//       Syntax (using `move`)
//          let clsre = move |...| { body }
//  2. Capturing a Reference: Borrowing immutably all the captured values it uses, which is the **default**.
//       Syntax (as normal):
//          let clsre = | ...| { body }
//  3. Capturing a Mutable Reference: Borrowing mutably of all the captured mutable values it uses.
//       Syntax (using `mut`):
//          let mut clsre = | ...| { body }
//
// The closure decides which of these to use based on what its body does with the captured values.
//

fn closure_move_and_copy() {
  // x owns an i32 which implements Copy, and xs owns a Vec which does not.
  let x: i32 = 2;
  let xs: Vec<i32> = vec![1, 2, 3];

  // this closure performs a Move of xs, and a Copy of x.
  let fn_move = move || { print!("{x:?}, {xs:?}") };

  // can't use moved value
  // println!("{xs:?}"); // Error!

  // can use copied value
  println!("{x:?}");
}

fn closure_borrow_immut() {
  // x owns an i32 which implements Copy, and xs owns a Vec which does not.
  let x: i32 = 2;
  let xs: Vec<i32> = vec![1, 2, 3];

  // this closure performs a (Immutable) Borrow of xs (and a Borrow or Copy of x).
  let fn_borrow_immut = || { print!("{x:?}, {xs:?}") };

  fn_borrow_immut();
  // can reuse the borrowed value after clsre_borrow has finished (immutably) using it
  println!("After calling closure: {xs:?}");
}

fn closure_borrow_mut() {
  // x owns an i32 which implements Copy, and xs *mutably* owns a Vec which does not.
  let x: i32 = 2;
  let mut xs: Vec<i32> = vec![1, 2, 3];

  // this closure performs a Mutable Borrow of xs (and a Borrow or Copy of x).
  let mut fn_borrow_mut = || { print!("{:?}", xs.push(x)) };

  fn_borrow_mut();
  // Can reuse the borrowed mutable value after clsre_borrow has finished using (and possibly mutating) it
  println!("After calling closure: {xs:?}");
}