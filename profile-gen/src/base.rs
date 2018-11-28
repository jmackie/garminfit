lazy_static! {
    /// Names of base types we are aware of, as defined in the SDK.
    pub static ref KNOWN_BASE_TYPES: std::collections::HashSet<String> = {
        [
            "enum", "sint8", "uint8", "sint16", "uint16", "sint32", "uint32",
            "float32", "float64", "uint8z", "uint16z", "uint32z", "sint64",
            "uint64", "uint64z","string", "byte", "bool"
        ].iter().cloned().map(String::from).collect()
    };
}
