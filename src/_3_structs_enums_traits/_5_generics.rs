// -----------------------------------------------
// # GENERICS
//
// Generic types <T : Trait1 + Trait2 + ...> let us parameterise functions, datatypes (structs and enums), and traits,
// by abstract type variables T that can be trait-bounded to implement the traits in Trait1 + Trait2 + ....
//
// Trait bounds let us write code that uses generic type parameters that must have a particular behavior.
// The compiler uses the trait bound information to check that all the concrete types used provide the correct behavior.
//
// Syntax:
//
//   f<T : Trait1 + Trait2 + ...>(x : T)
//
// Syntax sugar 1 (impl syntax):
//
//   f(x : impl (Trait1 + Trait2 + ...))
//
// Syntax sugar 2 (where syntax):
//
//   f<T, ...>(x : T)
//   where
//     T: Trait1 + Trait2 + ...,
//


// -----------------------------------------------
// ## Generics in Functions
//
// Syntax:
//
// fn f<T : Traits, ...>(x : T) {
//    ...
// }
fn generics_in_functions<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
      if item > largest {
          largest = item;
      }
  }
  largest
}

fn generics_in_functions_implsugar(list: &[impl PartialOrd]) -> &impl PartialOrd {
  generics_in_functions(list)
}

fn generics_in_functions_wheresugar<T>(list: &[T]) -> &T
where
  T: PartialOrd
{
  generics_in_functions(list)
}


// -----------------------------------------------
// ## Generics in Structs
//
// Syntax:
//
// struct StructName<T : Traits, ...> {
//   ...
// }
struct Point<T : PartialOrd> {
  x: T,
  y: T,
}

fn using_generic_structs(p1 : Point<i32>, p2 : Point<i32>) -> Point<i32>{
  // Note: (+) is sugar for a regular function `add(self : T, rhs : T)`, so will cause MOVE of ownership unless T implements COPY.
  // Although Point is not copyable, i32 is copyable, so below passes a copy of p1.x and p1.y from p1. Likewise for p2.
  let sum1: i32 = p1.x + p1.y;
  let sum2: i32 = p2.x + p2.y;
  if sum1 >= sum2 {
     p1
  }
  else {
     p2
  }
}

// #### More complex use of Generics in Structs
//
// fn largest_point<T : PartialOrd
//                    + std::ops::Add<Output = T>  // Output is an associated type of the Add trait that specifies the result type of add().
//                    + Copy>
//                 (p1 : Point<T>, p2 : Point<T>) ->  Point<T> {
//   // Compute the sums of x and y for each point.
//   let sum1 = p1.x + p1.y;
//   let sum2 = p2.x + p2.y;
//   if sum1 >= sum2 {
//      p1
//   }
//   else {
//      p2
//   }
//   // Note about Copy:
//   // (+) is sugar for a regular function `add(self : T, rhs : T)`, so will cause MOVE of ownership unless T implements COPY.
//   //      let sum1 = p1.x.add(p1.y);
//   //      let sum2 = p2.x.add(p2.y);
//   // Hence for non-copyable T, using p1.x and p1.y as arguments for `self` and `rhs` will each cause a partial move of p1.
// }

// fn largest_point_wheresugar<T>(p1 : Point<T>, p2 : Point<T>) -> Point<T>
// where
//   T: PartialOrd + std::ops::Add<Output = T> + Copy,
// {
//   largest_point(p1, p2)
// }


// -----------------------------------------------
// ## Generics in Enums
//
// Syntax:
//
// enum EnumName<T : Traits, ...> {
//   ...
// }

enum Result<T, E> {
  Ok(T),
  Err(E)
}

fn using_generic_enums() -> Result<i32, std::num::ParseIntError>{
  let number_str = "10";
  let number = match number_str.parse::<i32>() {
      Ok(number)  => number,
      Err(e) => return Result::Err(e),
  };
  Result::Ok(number)
}
