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
// We can use generics to define abstract (trait-bounded) struct types.
// These can be used (and perhaps specialised) to define struct methods and regular functions.
//
// Syntax:
//
//    struct StructName<T : Traits, ...> {
//      ...
//    }
//
//    impl<T : Traits, ...> StructName<T, ...> {
//      fn method_name(&self, ...) -> {
//      }
//    }
//
struct Point<T, U>{
  x : T,
  y : U
}

// Ad-hoc methods for specialised structs
impl Point<f32, f32> {
  fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

// Generic methods for abstract structs
impl<T : PartialOrd> Point<T, T> {
  // warning: will cause move of ownership unless T and Point<T, T> implement copy
  fn largest_coord(self) -> T {
      if self.x > self.y  {self.x} else {self.y}
  }
}

// Ad-hoc functions for specialised structs
fn using_generic_structs1(p1 : Point<i32, i32>, p2 : Point<i32, i32>) -> Point<i32, i32>{
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

// Generic functions for abstract structs
fn using_generic_structs2<T : PartialOrd
                   + std::ops::Add<Output = T>  // Output is an associated type of the Add trait
                   + Copy>
                (p1 : Point<T, T>, p2 : Point<T, T>) ->  Point<T, T> {
  // Compute the sums of x and y for each point.
  let sum1 = p1.x + p1.y;
  let sum2 = p2.x + p2.y;
  if sum1 >= sum2 { p1 } else { p2 }
}
// Note about Copy:
// (+) is sugar for a regular function `add(self : T, rhs : T)`, so will cause MOVE of ownership unless T implements COPY.
//      let sum1 = p1.x.add(p1.y);
//      let sum2 = p2.x.add(p2.y);
// Hence for non-copyable T, using p1.x and p1.y as arguments for `self` and `rhs` will each cause a partial move of p1.

// -----------------------------------------------
// ## Generics in Enums
//
// We can use generics to define abstract (trait-bounded) enum types.
// These can be used (and perhaps specialised) to define enum methods and regular functions.
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

// Ad-hoc methods for specialised enums
impl<E> Result<u32, E> {
  fn from_ok_u32(self) -> u32 {
    match self {
      Result::Ok(res) => res,
      Result::Err(_) => 0
    }
  }
}

// Generic methods for abstract enums
impl<T, E> Result<T, E> {
  fn from_ok(self, default : T) -> T {
    match self {
      Result::Ok(res) => res,
      Result::Err(_) => default
    }
  }
}

// Ad-hoc functions for specialised enums
fn using_generic_enums1() -> Result<i32, std::num::ParseIntError>{
  let number_str: &str = "10";
  let number: i32 = match number_str.parse::<i32>() {
      Ok(number)  => number,
      Err(e) => return Result::Err(e),
  };
  Result::Ok(number)
}

// Generic functions for abstract enums
fn using_generic_enums2<T : std::ops::Add
                          + std::ops::Add<Output = T>
                          + Clone
                          ,
                        E : Clone>
                (res1 : &Result<T, E>, res2 : &Result<T, E>) -> Result<T, E> {
  // Because res1 and res2 are shared references, so are their contents in Ok(..) and Err(..).
  match (res1, res2) {
    // We need T and E to be clonable or copyable, as we only have references available (no owners) and we want to use
    // their underlying values to construct a new value that we return ownership to.
    (Result::Ok(x1ref), Result::Ok( x2ref)) => {
      let x1: T = (*x1ref).clone();
      let x2: T = (*x2ref).clone();
      Result::Ok(x1 + x2)
    },
    (Result::Err(e), _) => Result::Err((*e).clone()),
    (_, Result::Err(e)) => Result::Err((*e).clone())
  }
}

// -----------------------------------------------
// ## Generics in Traits
//
// We can use generics to define abstract (bounded) traits.
// Whereas regular traits let us declare trait methods for a single abstract type `Self`,
//         generic traits let us declare trait methods for further abstract types, <T, ...>, which
//          can be related or unrelated to the type `Self.`
//
// Syntax:
//
//    trait TraitName<T : Traits, ...> {
//      ...
//    }
//
//    impl<T : Traits, ...> TraitName<T, ...> for StructOrEnum {
//      ...
//    }

trait Decrement<T> {
  fn decrement(&self) -> T;
}

// the use of abstract traits doesn't really come in handy here:
impl Decrement<i32> for i32 {
  fn decrement(&self) -> i32 {
      self - 1
  }
}
// this says that `decrement` can return i32 for a Point<i32, T>.
impl<T> Decrement<i32> for Point<i32, T> {
  fn decrement(&self) -> i32 {
      self.x - 1
  }
}
// this says that `decrement` can return f32 for a Point<i32, T>.
impl<T> Decrement<f32> for Point<i32, T> {
  fn decrement(&self) -> f32 {
      (self.x - 1) as f32
  }
}
// this says that `decrement` can return Point<i32, T> for Point<i32, T>.
impl<T : Copy> Decrement<Point<i32, T>> for Point<i32, T> {
  fn decrement(&self) -> Point<i32, T> {
      Point {x : self.x - 1
           , y : self.y }
  }
}
