// ----------------------------------------
// # ITERATORS
//
// A struct is an iterator if it holds an iterator state which we can perform some task on as sequence of elements.
//
// The Iterator trait is used to implement iterators for specific strucys.
// It requires two things:
//    1. The `Item` type, which is the type of elements being iterated over
//    2. The `next` method, which tries to return the next element, otherwise returning None when iteration ends.
//
//    pub trait Iterator {
//      type Item;
//      fn next(&mut self) -> Option<Self::Item>;
//
//      // there are some provided default methods:
//
//    }
//

// ## Iterators: Implementing an Iterator, via the Iterator Interface
//
// Creating an iterator involves two steps:
//   1. Creating a struct to hold the iterator's state
//   2. Implementing the `Iterator` for that struct.

// Struct for a Sequence of Fibonacci numbers.
struct Fibonacci {
  curr: u32,
  next: u32,
}

// Implements `Iterator` for `Fibonacci`.
impl Iterator for Fibonacci {
  // We can refer to this type using Self::Item
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
      let current: u32 = self.curr;

      self.curr = self.next;
      self.next = current + self.next;

      // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
      // will never return `None`, and `Some` is always returned.
      Some(current)
  }
}

// ## Iterators: Creating Iterators from Collection Datatypes (from the standard library)
//
// Structs T that are collections in the standard library, often
// come with three common methods for creating iterators from that collection:
//
//   1. iter(), which iterates over &T.
//   2. iter_mut(), which iterates over &mut T.
//   3. into_iter(), which iterates over T.
//
//  (These are methods directly provided by collections (like Vec<T> or slices), and is not part of any trait.)

// ## Iterators: Laziness
//
// Iterators are lazy, so do not compute or produce values immediately when they're created, or
// when we call methods like map, filter, or collect.
// Instead, the work is deferred until the iterator is consumed, typically in a loop or by a terminal operation.
// no effect until you call explicit methods that consume the iterator.