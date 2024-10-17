// -----------------------------------------------
// # TRAITS
//
// Traits are a collection of:
//   1. methods
//   2. associated functions
//   3. associated types
// for an unknown type `Self` that any type can provide an implementation of.
//
// Combined with generics (later), trait bounds then let us specify abstract types as implementing these traits.
//
//    trait TraitName {
//      type Assoc_Type_Name;
//
//      fn assoc_fun_name(...) -> ...;
//      fn method_name(self : Self, ...);
//      fn default_method_name(self : Self, ...) {
//
//      }
//    }
//
//    impl TraitName for Type {
//      type Assoc_Type_Name = Specific_Assoc_Type;
//      fn assoc_fun_name(...) -> ... {
//        ...
//      }
//      fn method_name(self : Self, ...) {
//        ...
//      }
//    }
//
// Traits in Rust can be thought of as type classes in Haskell with a type parameter `Self`:
//    type class ClassName selftype where
//        method_name :: selftype -> ...
//

// -----------------------------------------------
// ## Defining Traits
//
// Syntax:
//
//    trait TraitName {
//      fn assoc_fun_name(...) -> ...;
//      fn method_name(&self, ...);
//      fn default_method_name(&self, ...) {
//
//      }
//    }
//
trait Show {
  type AltShowType;
  fn alt_show(s : String) -> Self::AltShowType;
  // Method
  fn show(&self) -> String;
  // Default method
  fn show_twice(&self) -> String{
    // Create a mutable owner for a string on the heap
    let mut owned_string: String = self.show();
    // Create a slice reference to a string literal (on the stack)
    let borrowed_string: &str = "world";
    // Mutate the string on the heap.
    owned_string.push_str(borrowed_string);
    // Return ownership of string on the heap.
    return owned_string;
  }
}

// -----------------------------------------------
// ## Implementing Traits
//
// Syntax:
//
//    impl TraitName for Type {
//      fn assoc_fun_name(...) -> ... {
//        ...
//      }
//      fn method_name(self : Self, ...) {
//        ...
//      }
//    }
//
// Using Traits
//
//
struct User {
  active: bool,
  sign_in_count: u64,
  username: String,
}

impl Show for User {
  type AltShowType = Vec<char>;
  fn alt_show(s : String) -> Self::AltShowType {
    let mut v: Vec<char> = vec![];
    for c in s.chars() {
      v.push(c);
    }
    v
  }
  fn show(&self) -> String {
    self.username.to_string()
  }
}

pub fn using_traits_example(){
  let user1: User = User {
    active:true,
    sign_in_count:0,
    username:String::from("hello")
  };
  let s: String = user1.show_twice();
  print!("{s}");
  let v: Vec<char> = User::alt_show(user1.show());
  print!("{v:?}");
}