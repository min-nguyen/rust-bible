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
// We can use generics to define abstract (bounded) function types i.e. their argument and return types
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
// We can use generics to define abstract (bounded) struct types
//
// Syntax:
//
// struct StructName<T : Traits, ...> {
//   ...
// }
struct Point1<T> {
  x: T,
  y: T,
}

fn using_generic_structs(p1 : Point1<i32>, p2 : Point1<i32>) -> Point1<i32>{
  // Note: (+) is sugar for a regular function `add(self : T, rhs : T)`, so will cause MOVE of ownership unless T implements COPY.
  // Although Point1 is not copyable, i32 is copyable, so below passes a copy of p1.x and p1.y from p1. Likewise for p2.
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
//                 (p1 : Point1<T>, p2 : Point1<T>) ->  Point1<T> {
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
// We can use generics to define abstract (bounded) enum types
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
  let number_str: &str = "10";
  let number: i32 = match number_str.parse::<i32>() {
      Ok(number)  => number,
      Err(e) => return Result::Err(e),
  };
  Result::Ok(number)
}

// -----------------------------------------------
// ## Generics in Traits
//
// We can use generics to define abstract (bounded) traits
//
// Syntax:
//
// trait TraitName<T : Traits, ...> {
//   ...
// }

trait Greatest<T> {
  fn greatest(&self) -> T;
}


// -----------------------------------------------
// ## Generics in Method Definitions (for Structs, Enums, and Traits)
//
// We can use generics to define methods for a Struct<T>, Enum<T>, or Trait<T> of a specific type T.
//
// Syntax for Structs/Enums:
//
//  struct<T : Traits, ...> StructOrEnum<T, ...> {
//   ...
//  }
//
//  impl<T  : Traits, ...> StructOrEnum<T, ...> {
//    ...
//  }
//
// Syntax for Traits:
//
//  trait Trait<T : Traits, ...> {
//    ...
//  }
//
//  impl<T> Trait<T : Traits, ...> for Type< {
//    ...
//  }

// Structs, for example:
struct Point2<T, U>{
  x : T,
  y : U
}

impl Point2<f32, f32> {
  fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

impl<T, U> Point2<T, U> {
  // warning: will cause move of ownership unless T, U, and Point2<T, U> implement copy
  fn mixup(self, other: Point2<T, U>) -> Point2<T, U> {
      Point2 {
          x: self.x,
          y: other.y,
      }
  }
}

impl<T : PartialOrd> Point2<T, T> {
  // warning: will cause move of ownership unless T and Point2<T, T> implement copy
  fn largest_coord(self) -> T {
      if self.x > self.y  {self.x} else {self.y}
  }
}

// Traits, for example:
// trait Greatest<T> {
//   fn greatest(&self) -> T;
// }
impl Greatest<i32> for Point1<i32> {
  fn greatest(&self) -> i32 {
      if self.x > self.y {self.x} else {self.y}
  }
}
