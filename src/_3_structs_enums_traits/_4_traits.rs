// -----------------------------------------------
// # TRAITS
//
// Traits are a collection of methods that any type can provide an implementation of.
// Combined with generics (later), trait bounds then let us specify abstract types as implementing these traits.
//
//    trait TraitName {
//      fn method_name(&self, ...);
//      fn default_method_name(&self, ...) {
//
//      }
//    }
//
//    impl TraitName for Type {
//      fn method_name(&self, ...) {
//        ...
//      }
//    }
//
//


// -----------------------------------------------
// ## Defining Traits
//
// Syntax:
//
//    trait TraitName {
//      fn method_name(&self, ...);
//      fn default_method_name(&self, ...) {
//
//      }
//    }
//
trait Show {
  fn show(&self) -> String;
  fn show_twice(&mut self) ->  String{
    let mut owned_string: String = self.show().to_owned();
    let borrowed_string: &str = "world";
    owned_string.push_str(borrowed_string)
  }
}

// -----------------------------------------------
// ## Implementing Traits
//
// Syntax:
//
//    impl TraitName for Type {
//      fn method_name(&self, ...) -> ... {
//
//      }
//    }
//

struct User {
  active: bool,
  sign_in_count: u64,
  username: String,
}
impl Show for User {
  fn show(&self) -> String {
    self.username.to_string()
  }
}
