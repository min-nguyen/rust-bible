// -----------------------------------------------
// # ENUMS
//
// An enum represents a sum of values, each corresponding to a possible variant containing different data.
// It is represented in memory as:
//   1. a discriminant that indexes which variant is being used
//   2. the data which the variant contains.
// The size of the enum is determined by the total size of its largest variant (plus the size of the discriminant)
//
//    enum EnumName { Variant1Name( arg_type, ...),
//                    Variant2Name{ field_name: field_type, ... }
//                  , ... }
//
//    let x = EnumName::Variant1Name(arg_value, ...);
//    let y = EnumName::Variant2Name{field_name: field_value, ...};

enum Message {
  Quit,                         //  Quit has no associated data
  Move { x: i32, y: i32 },      //  Move has named fields for two  i32 values
  Write(String),                //  Write contains a single { ptr, len, cap } value that manages a string on the heap
  ChangeColor(i32, i32, i32),   //  ChangeColor includes three i32 values
}

// -----------------------------------------------
// ## Using Enums
fn enums_usage(){
  // [Specifying enum values]
  let quit_message: Message = Message::Quit;
  let move_message: Message = Message::Move { x: 0, y: 0 };
  let write_message: Message = Message::Write(String::from("hello"));
  let write_message: Message = Message::ChangeColor(0,5,7);
}

// -----------------------------------------------
// ## Pattern matching
//
// The `match variable { pattern => code, ... }` syntax lets us pattern match on a types as being a particular values,
// and also give variable names to those values and any of their contained data.
//
// The Rules of Matching:
//    * Matching must be exhaustive over all values of the type.
//    * Matching must be consistent in the return type for each pattern.
//    * Matching a pattern as a variable sets that variable as an owner
//      (in the same way as that a function's parameter names are used to "match" against the provided arguments).
//
// Syntax:
//    * The `x@pattern => ...` syntax lets us set a variable name x that owns the value pattern matched on.
//    * The `_` syntax lets us match any pattern/value and not assign a variable to it.
fn matching(n : i32) {

  // Matching on basic values.
  let msg: Message = match n {
    // Matches on the i32 value 0
    0      => Message::Quit,
    // Matches on any i32 value as the variable m
    m => Message::Move { x: m, y: m },
  };

  // Matching on enums.
  let msg1: Message = match msg {
    // Matches `Move` with any i32 values as the variables x_val and y_val
    Message::Move{x : x_val, y: y_val}
      => Message::Move{x : x_val, y: y_val},
    // Matches `Write` with any String value as the variable s
    Message::Write(s)
      => {
      println!("{s}");
      Message::Write(s)
    },
    // Matches Color with any i32 values as variables r g b
    Message::ChangeColor(r,g,b)
      => Message::ChangeColor(r,g,b),
    // Matches Quit
    Message::Quit
        => Message::Quit,
  };

  // Syntax sugar.
  let msg2: Message = match msg1 {
    //  The `x@pattern => ...` syntax lets us assign a variable name x to a value pattern.
    q_msg@Message::Quit
        => q_msg,
    //  The `_` syntax lets us match any value and not assign a variable to it.
    Message::Move{x : _, y: _}
        => Message::Move{x : 0, y: 0},
    //  likewise:
    _
        => Message::Write(String::from("Default"))
  };
}

// -----------------------------------------------
// ## Pattern matching: References and Borrowing
//
// Recall one of the rules of pattern matching:
//    * Matching a value as a variable sets that variable as its owner i.e. causes a copy or move.
//
// The `ref x` syntax lets us declare a variable x as a reference instead.
//    The following 2 lines are equivalent:
//      let x = &v;
//      let ref x = v;
// We need `ref x` (instead of x :&T = &v) if we are directly matching on a value v, and
// we want to borrow the value instead of copying or moving its ownership.
//      let m : Maybe<u8> = ...;
//      match m as {
//        case Just(ref x) => ... // x is a reference to a u8 value
//      };
//
// Alternatively, we could create a reference for the value beforehand, whereby matching on that reference will
// *automatically* create references for (and borrow) any of its contained values.
//      let m : Maybe<u8> = ...;
//      match &m as {
//        case Just(x) =>     ... // x is a reference to a u8 value
//      };

fn matching_with_refs(msg : Message){
  // Matching a value without `ref`
  let msg1: Message = match msg {
    // x_val and y_val own the i32 values x and y copied from msg
    Message::Move{x : x_val, y: y_val}
      // below returns x_val and y_val as values in msg1
      => Message::Move{x : x_val, y: y_val},
    // s owns the String value (partially) moved from msg
    Message::Write(s)
      // below *moves* s's ownership of the String value to msg1
      => Message::Write(s),
    _
      => Message::Quit
  };

  // We cannot match on `s` in the partially moved msg, because s's ownership was moved to msg1
  // let x = match msg {
  //   Message::Write(s) => (), // Error: use of moved value in msg
  //   _ => ()
  // };

  // Matching a value with `ref`
  let msg2: Message = match msg1 {
    // x_ref and y_ref are references that borrow the i32 values x and y from msg1
    Message::Move{x : ref x_ref, y: ref y_ref}
      // below dereferences and returns their underlying values in msg2
      => Message::Move{x : *x_ref, y: *y_ref},
    // s is a reference that borrows the String value from msg
    Message::Write(ref s)
      // below has to read the value of s and create a new String, as it does not own an underlying string
      => Message::Write(s.to_string()),
    // msg is a reference that borrows the entire msg1 value
    ref msg
      => Message::Quit
  };

  // Matching a reference (without `ref`)
  let msg3: Message = match &msg2 {
    // x_ref and y_ref are references that borrow the i32 values x and y from msg2
    Message::Move{x : x_ref, y: y_ref}
      // below dereferences and returns their underlying values in msg3
      => Message::Move{x : *x_ref, y: *y_ref},
    // s is a reference that borrows the String value from msg2
    Message::Write( s)
      // below has to read the value of s and create a new String, as it does not own an underlying string
      => Message::Write(s.to_string()),
    // msg is a reference that borrows the entire msg2 value
    msg
      => Message::Quit
  };
}

// -----------------------------------------------
// ## Pattern matching: If-Let
//
// The `if let pattern = variable { code  }` lets us match a variable as a single pattern while ignoring the rest, and
// only execute a block of code if that match was successful.

fn matching_with_if_let(msg : Message){
  // Matching a value using if-let (without `ref`)
  let msg1 : Message = if let Message::Write( s) = msg {
      print!("{s}");
      // below *moves* s's ownership of the String value to msg1
      Message::Write(s)
  } else { Message::Quit };

  // We cannot assign msg to msg2, because it was partially moved when moving s's ownership to msg1
  // let msg2 = msg; // Error: msg was partially moved

  // Matching a value using if-let (with `ref`)
  let msg2 : Message = if let Message::Write(ref s) = msg1 {
      print!("{s}");
      // below has to read the value of s and create a new String, as it does not own an underlying string
      Message::Write(s.to_string())
  } else { Message::Quit };

  // Matching a reference using if-let  (without `ref`)
  let msg3 : Message = if let Message::Write( s) = &msg2 {
    print!("{s}");
    // below has to read the value of s and create a new String, as it does not own an underlying string
    Message::Write(s.to_string())
  } else { Message::Quit };
}
