// -----------------------------------------------
// # ENUMS
//
// An enum represents a sum of values, each corresponding to a possible variant.
// It is represented in memory as ...
//
//    enum EnumName { VariantName(arg_type, ...),
//                  , ... }
//
//    let x = EnumName::VariantName(arg_value, ...);
//