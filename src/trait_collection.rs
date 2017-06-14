/// Describes a colletion containing a type.
/// Types implementing this must guarantee that Default::default creates an
/// empty collection.
pub trait Collection {
    type Item;
}
