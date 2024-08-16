// -----------------------------------------------
// # ENUMS
//
// An enum represents a sum of values, each corresponding to a possible variant containing different data.
// It is represented in memory as a "discriminant" that indexes which variant is being used, alongside the data
// which the variant contains. The size of the enum is determined by the size of its largest variant (plus size of
// the discrimnant)
//
//    enum EnumName { VariantName(arg_type, ...),
//                  , ... }
//
//    let x = EnumName::VariantName(arg_value, ...);
//
