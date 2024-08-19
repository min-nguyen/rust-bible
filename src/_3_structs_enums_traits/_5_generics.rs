// -----------------------------------------------
// # GENERICS
//
// Generic types <T : ImplT> let us parameterise functions, datatypes (structs and enums), and traits (covered later)
// by an abstract type variable T that must provide an implementation of the `impl` block ImplT.
//

// -----------------------------------------------
// ## Generics in Functions
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}
