
// -----------------------------------------------
// # LIFETIMES
//
// Reference lifetimes are used by the borrow checker to ensure all references are guaranteed to point to a valid value.
//
// Most of the time lifetimes are implicit and inferred. We must explicitly annotate lifetimes when they could be related in different ways.
// Using explicit lifetimes requires generics.
//
// Syntax for lifetimes in types:
//   &'a T           //  the reference to a value of type T has the lifetime 'a
//
// Syntax for lifetimes as parameters:
//   foo<'a, 'b, ... //  the lifetime parameter 'a must live at least as long as `foo`
//


// -----------------------------------------------
// ## LIFETIMES IN FUNCTIONS
//
// Functions use lifetimes mainly to return references that are guaranteed to point to some data.
// Functions using lifetimes have 2 rules:
//   1. All references must have an annotated lifetime.
//   2. All references returned must have the same lifetime as an input or be static.
//
// Syntax:
//    fn foo<'a, 'b>(xref : &a' T, ...) -> &a' U { ... }

// This function is invalid, as 'a must live longer than the function, but &String::from("foo")` would create a
// reference to a `String` whose data is dropped upon exiting the function's scope:
//   fn invalid_output<'a>() -> &'a String { &String::from("foo") }

// This function takes as two i32 references with lifetimes 'a and 'b that outlive the function.
// It returns a reference with lifetime 'a which corresponds to one of the inputs' lifetimes.
fn lifetimes_in_functions<'a, 'b>(xref: &'a i32, yref: &'b mut i32) -> &'a i32{
  *yref += 1;
  println!("`print_multi`: x is {}, y is {}", xref, yref);
  &xref
}

fn main1() {
  let x: i32 = 7;
  let mut y: i32 = 9;

  let zref: &i32 = lifetimes_in_functions(&x, &mut y);
}

// -----------------------------------------------
// ## LIFETIMES IN STRUCTS AND ENUMS
//
// Structs and enums use lifetimes mainly to contain references that are guaranteed to always be valid.
// Their methods can also use lifetime annotations, either independently or relatedly to the struct or enum.
//
// Syntax:
//    struct StructName<'a, ...> { xref : &'a T, ... }
//
//    impl<'a, ...> StructName<'a, ...> {
//      fn method_name(&self, ...) -> {
//      }
//    }

// This struct contains two references that must outlive the structure
#[derive(Debug)]
struct NamedBorrowed<'a> {
  x: &'a i32,
  y: &'a mut i32,
}

impl<'a> NamedBorrowed<'a> {
  // This method refers to the lifetime parameter of the struct.
  fn add_one(&'a mut self) { (*self.y) += 1; }
  // This associated function declares a new lifetime parameter
  fn makeStruct<'b>(x : &'b  i32, y : &'b mut i32) -> NamedBorrowed<'b>{
      NamedBorrowed {x , y}
  }
}

fn main2() {
  let x = 18;
  let mut y = 15;
  let double = NamedBorrowed { x: & x, y: &mut y };
  println!("x and y are borrowed in {:?}", double);
}

// -----------------------------------------------
// ## LIFETIMES IN TRAITS
//
// Traits use lifetime annotations to work with methods and data structures that involve borrowing.
// Their implementations can also use lifetime annotations, either independently or relatedly to the trait.
//
// Syntax:
//
//    trait TraitName<'a, ...> {
//        fn method(self: Self) -> T;
//    }
//
//    impl<'a> TraitName<'a, ...> for StructOrEnum {
//        fn method(self: Self) -> T {
//
//        }
//    }

// Trait Definition with a Lifetime Parameter:
trait Summary<'a> {
  fn summarize(&self) -> &'a str;
}

// A struct with annotation of lifetimes.
struct Article<'a> {
  title: &'a str,
  content: &'a str,
}

// Trait Implementation for the Struct:
impl<'a> Summary<'a> for Article<'a> {
  fn summarize(&self) -> &'a str {
      self.title
  }
}

fn main3() {
    let article: Article = Article {title: "hello", content: "world"};
    let title: &str = article.summarize();
    println!("Article article is {:?}", title);
}