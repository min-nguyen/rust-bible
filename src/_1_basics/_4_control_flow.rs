
// # Control flow
fn control_flow() {
  // ## If(-Else) Expressions
  let x: bool =
      if 3 < 5 {
          println! ("3 < 5");
          true
      } else {
          println! ("3 > 5");
          false
      };

  // ## Loops
  // These recurse forever until explicitly broken out of.
  //  1. Using return will exit the current function.
  //  2. Using break will only exit the current loop.
  //  3. Using continue will skip to the next loop iteration.
  // We can also specify loop labels with a single quote, 'loop_label, to indicate which loop to `break` or `continue` to.
  let mut count: i32 = 0;
  'counting_up: loop {            // Label this loop as "counting_up"
      println!("count = {count}");
      let mut remaining: i32 = 5;

      loop {
          println!("remaining = {remaining}");
          if remaining == 0 {
              break;             //  Break out of inner loop, to reset `remaining` to 5
          }
          if count == 3 {
              break 'counting_up; // Break out of outer loop, to return the final counter
          }
          remaining -= 1;
      }
      count += 1;
      println!("End count = {count}");
  }

  // ## While loops
  let mut number = 3;
  while number != 0 {
      println!("{number}!");
      number -= 1;
  }

  // ## For loops
  let arr = [10, 20, 30, 40, 50];

  for element in arr {
      println!("the value is: {element}");
  }
}
