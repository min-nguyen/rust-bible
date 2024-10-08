// To build main.rs:
// > cargo build

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
    pub mod _4_traits;
}

// mod submodule2;
fn main() {
    _3_datatypes_and_traits::_4_traits::using_traits_example();
}
