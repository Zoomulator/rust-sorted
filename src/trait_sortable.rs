use super::Collection;
use std::cmp::Ordering;

/// Implements an interface for collections that can be sorted.
///
/// A collection that implements this trait must
/// be able to re-order its elements in a specific order (sorting).
pub trait Sortable: Collection {
    fn sort<F>(&mut self, f: F)
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering;
}
