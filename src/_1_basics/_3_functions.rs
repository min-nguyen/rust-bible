
// # Functions
fn functions() -> () {
  // ## Statements and expressions
  // Statements are instructions that perform some action and do not return a value, and are separated by ';'.
  // Expressions evaluate to a resultant value, and do not finish with a ';'
  let y = {      //                           }
      let x = 3; // } statement  }            }
      x + 1           // } expression } expression }
  };                  //                           } statement

  // ## Functions with return values
  // Functions with return values must declare their return type with `-> T``. Functions with no return value are implicitly `-> ()`.
  // The return value of the function is either:
  //   1. Implicitly the final expression in the block of the body of a function.
  //      This must not finish with a ';' as that implies a  statement.
  //   2. Explicitly a returned expression, which exits the function early.
  fn add_one(n : i32) -> i32 {
      if n > 0 {
          return n + 1; // return early
      };
      n + 2 // return normally
  }
}