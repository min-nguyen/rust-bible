// -----------------------------------------------
// # METHODS
//
// A method is a function defined within the context of a struct or enum,  (or trait not covered yet).
// The first parameter of a method is always `self: &Self` which represents the struct instance the method is called on.
//
//    impl StructName/EnumName/TraitName {
//      fn method_name(&self, ...) -> ... {
//
//      }
//    }
//

struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
      self.width * self.height
  }
}