// ----------------------------------------
// # ITERATORS
//
// An iterator allows you to perform some task on a sequence of items in a collection.
// They are lazy, so have no effect until you call methods that consume the iterator.
//
// The Iterator trait is used to implement iterators for specific collections.
// It requires two things:
//    1. The `Item` type, which is the type of elements being iterated over
//    2. The `next` method, which tries to return the next element, otherwise returning None when the iterator finishes.
//
//    pub trait Iterator {
//      type Item;
//      fn next(&mut self) -> Option<Self::Item>;
//
//      // there are some provided default methods:
//
//    }
//

struct Fibonacci {
  curr: u32,
  next: u32,
}

// Implement `Iterator` for `Fibonacci`.
impl Iterator for Fibonacci {
  // We can refer to this type using Self::Item
  type Item = u32;

  // Here, we define the sequence using `.curr` and `.next`.
  // The return type is `Option<T>`:
  //     * When the `Iterator` is finished, `None` is returned.
  //     * Otherwise, the next value is wrapped in `Some` and returned.
  fn next(&mut self) -> Option<Self::Item> {
      let current: u32 = self.curr;

      self.curr = self.next;
      self.next = current + self.next;

      // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
      // will never return `None`, and `Some` is always returned.
      Some(current)
  }
}