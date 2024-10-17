// ----------------------------------------
// # ITERATORS
//
// A struct is an iterator if it holds an iterator state which we can perform some task on as sequence of elements.
//
// The `Iterator` trait defines the behavior of all iterators in Rust.
//
// It requires two things:
//    1. The `Item` type.
//       This is the type of elements being iterated over.
//    2. The `next(&mut self) -> Option<Self::Item>` method.
//       This consumes (mutates) the iterator's internal state by advancing it forward, and element it returns is no longer
//       available from that iterator in future next() calls.
//       Note that next(&mut self) does not consume the iterator itself, as it borrows it mutably.
//       ### Iterator Ownership
//        - If the iterator owns the elements, next() consumes the elements themselves (they are moved).
//        - If the iterator borrows the elements, next() does not consume the elements, but only returns references to them.
//
//    pub trait Iterator {
//      type Item;
//      fn next(&mut self) -> Option<Self::Item>;
//
//      // there are some provided default methods:
//
//    }
//
// Iterators are Lazy:
//   The key method of the Iterator trait, next(), only produces one element at a time when it is explicitly called.
//   An iterator hence doesn't perform any computation until next() consumes the iterator.
//   Since the Iterator trait itself defines how values are produced in a deferred manner, the trait is inherently lazy.
//   Any type that implements this trait, such as Iter, follows the same lazy behavior.
//

// ## Iterators: Implementing The `Iterator` Trait
//
// Creating an iterator involves two steps:
//   1. Creating a struct to hold the iterator's state
//   2. Implementing the `Iterator` for that struct.
//         a. Defining the `Item` type
//         b. Defining the `next(&mut self)` method

use std::{collections::vec_deque::Iter, iter::{Filter, Map}};

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


// ## Iterators: Concrete Iter<T>, IterMut<T>, and IntoIter<T> Structs
//
// The Iter<'a, T>,  IterMut<'a, T>, and IntoIter<'a, T> structs are concrete implementation of the `Iterator` trait.
// -  Iter<'a, T> borrows a collection T and lets us iterate over references to its elements
//       fn next(&mut self) -> Option<&T>;
// -  IterMut<'a, T> mutably borrows a collection T lets us iterate over mutable references to its elements
//       fn next(&mut self) -> Option<&mut T>;
// -  IntoIter<'a, T> takes ownership of (moves) a collection T lets us iterate and take ownership of its elements
//       fn next(&mut self) -> Option<T>;
//
// Structs T that are collections in the standard library _often_ come with three common methods
// for creating an Iter<'a, T>, from that collection:
//   1. iter(&self), which iterates over &T, hence borrowing T and its elements.
//      This returns an `Iter<'a, T>`
//   2. iter_mut(&mut self), which iterates over &mut T, hence mutably borrowing T and its elements.
//      This returns an `IterMut<'a, T>`
//   3. into_iter(self), which iterates over T, hence taking ownership of T and its elements.
//      This returns an `IntoIter<'a, T>`
//
//  (These are methods directly associated with structs like Vec<T> or slices &[U], and are not part of any trait.)

fn borrowing_iterator(){
  let v = vec![Box::new(1)];
  // Creates an iterator that borrows v1 and its elements
  let v_iter = v.iter();
  let v: Vec<&Box<i32>> = v_iter.map(|x: &Box<i32>| x ).collect();

  // We can't create a temporary Vec that is freed from the heap after the scope of the statement
  // and then use an iterator that borrows it:
  // let bad_borrowing_iter = vec![1,2,3].iter(); // Error: temporary vector [1,2,3] is dropped while borrowed.
  // let x : Vec<i32> = bad_borrowing_iter.map(|x| x + 1).collect();
}

fn mutably_borrowing_iterator(){
  let mut v: Vec<i32> = vec![1,2,3];
  // Creates an iterator that mutably borrows v1 and its elements
  let v_iter: std::slice::IterMut<'_, i32> = v.iter_mut();
  v_iter.map(|x: &mut i32| *x + 1);
}

fn owning_iterator(){
  let v: Vec<i32> = vec![1,2,3];
  // Creates an iterator that takes ownership of v1 and its elements
  let v_iter: std::vec::IntoIter<i32> = v.into_iter();
  v_iter.map(|x: i32| x + 1);
}


// ## Iterators: Other Concrete Iterator Structs
//
// Generic structs like Map<T> and Filter<T> are also iterators with their own methods.
// These are produced by calling methods like map() and filter().


// ## Iterators: "Consuming Adaptors" -- Methods that Consume the Iterator (Laziness)
//
// **Consuming adaptors** are methods fn(self) that:
//  - Take ownership of the iterator and iterate through the items by calling next(&mut self).
//    Hence makes the original iterator no longer usable.
//  - Note that `next(&mut self)` *within* any iterator still only mutably borrows each item.

// Consuming Adaptor: sum()
fn iterator_sum() {
  let v1: Vec<i32> = vec![1, 2, 3];
  let v1_iter: std::slice::Iter<'_, i32> = v1.iter();
  let total: i32 = v1_iter.sum();
}
// Consuming Adaptor: for_each()
fn iterator_for_each() {
  let v1: Vec<i32> = vec![1, 2, 3];
  let v1_iter: std::slice::Iter<'_, i32> = v1.iter();
  let unit : () = v1_iter.for_each(|x| println!("{}", x));
}
// Consuming Adaptor: collect()
fn iterator_collect() {
  let v1: Vec<i32> = vec![1, 2, 3];
  let v1_iter: std::slice::Iter<'_, i32> = v1.iter();
  let v1_refs: Vec<&i32> = v1_iter.collect();
}

// ## Iterators: "Iterator Adaptors" -- Methods that Produce Other Iterators (and Method Chaining)
//
// **Iterator adaptors**` are Methods defined on the Iterator trait.
// - They don't consume iterators and so do not perform any computation.
// - They instead produce new Iterators by nesting them.
// - To use the result of the iterator adaptors, they must eventually be consumed

// Iterator adaptor: map() and filter()
fn iterator_map_filter() {
  let v: Vec<i32> =  vec![1, 2, 3];

  // Creates an iterator (a concrete Iter<> struct) that borrows elements
  let v_iter : std::slice::Iter<i32>
    = v.iter();
  // Returns another iterator ( a concrete Map<> struct)
  let v_iter_map : Map <std::slice::Iter<i32> ,_>
    = v_iter.map(|x_ref: &i32| x_ref );
  // Returns another iterator ( a concrete Filter<> struct)
  let v_filter_map_iter: Filter<Map <std::slice::Iter<i32> ,_>, _>  =
      v_iter_map.filter(|x_ref_ref: &&i32| **x_ref_ref > 0);

  // Consumes all iterators and collects the resulting values into a collection datatype
  let v_refs : Vec<&i32> = v_filter_map_iter.collect();
}
