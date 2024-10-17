// To build lib.rs:
// > cargo build --lib

mod _1_basics {
     mod _1_variables;
     mod _2_datatypes;
     mod _3_functions;
     mod _4_control_flow;
}
mod _2_ownership {
    mod _2_owners_and_scope;
    mod _3_references_and_lifetimes;
    mod _4_slices;
}
mod _3_datatypes_and_traits {
    mod _1_structs;
    mod _2_enums;
    mod _3_methods;
    mod _4_traits;
}
mod _4_generics{
    mod _1_generic_types;
    mod _2_lifetime_annotations;
}
mod _5_functional_features{
    mod _1_closures;
    mod _2_iterators;
}

fn main() {
    // reference_example_2()
}
