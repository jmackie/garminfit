/// Trait for getting the value from a message field.
pub trait Field {
    type Value;
    fn value(&self) -> Self::Value;
}
