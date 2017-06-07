use super::Collection;

/// Marks a collection as retaining order when handling iterators.
///
/// A collection type implementing this must...
/// 1. retain the order of the elements consumed from an iterator.  
/// 2. retain the order in iterators it produces.  
pub trait RetainsOrder: Collection{
}