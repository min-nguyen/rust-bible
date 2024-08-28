// To build lib.rs:
// > cargo build --lib

mod _1_basics {
     mod _1_variables;
     mod _2_datatypes;
     mod _3_functions;
     mod _4_control_flow;
}
mod _2_ownership {
    mod _2_owners;
    mod _3_references;
    mod _4_slices;
}
mod _3_datatypes_and_traits {
    mod _1_structs;
    mod _2_enums;
    mod _3_methods;
    mod _4_traits;
}
mod _4_generics{
    mod _1_generics;
    mod _2_lifetimes;
}

// mod submodule2;
fn main() {
    // reference_example_2()
}
