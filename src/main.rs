
// mod submodule2;
fn main() {
    reference_example_2()
}

fn reference_example_2() {
    // mutable owner
    let mut s = String::from("hello");

    // mutable reference ref1_s is in scope
    let ref1  = &mut s;
    // let iref1 = &s; //  <-- Not allowed, as ref1 is still being used
    // s.push_str("s");  <-- Not allowed, as ref1 is still being used
    ref1.push_str("s");
    // mutable reference  ref1_s will not be used after this point

    // mutable reference  ref2_s is in scope
    let ref2 = &mut s;
    ref2.push_str("s");
    // mutable reference  ref2_s will not be used after this point

    // owner s can be used again
    s.push_str("s");
}


// fn reference() {
//     let s1 = String::from("hello");  // <<- Create an owner "s1" that points to "hello" allocated on the heap
//     let len = calculate_length(&s1); // <<-- Create and pass a reference "&s1" that points to "s1" allocated the stack.
//     println!("The length of '{s1}' is {len}.");
// }
// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()  // As s is a reference to s1, Rust will implicitly dereference it from &String to String
//              // so that it can call the len() method.
// } // Here, s goes out of scope.

