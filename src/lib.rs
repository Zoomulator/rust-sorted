//! Find it on [github](https://github.com/Zoomulator/rust-sorted).

#[macro_use]
pub mod macro_sort_order;

mod iter_difference;
mod iter_intersection;
mod iter_union;
mod sorted_iter;
mod std_slice;
mod std_vec;
mod trait_collection;
mod trait_retains_order;
mod trait_searchable_by_order;
mod trait_sort_order;
mod trait_sortable;
mod trait_sorted_insert;
mod trait_sorted_iterator;
mod type_sorted;

pub use iter_difference::DifferenceExt;
pub use iter_intersection::IntersectionExt;
pub use iter_union::UnionExt;
pub use sorted_iter::SortedIter;
pub use trait_collection::Collection;
pub use trait_retains_order::RetainsOrder;
pub use trait_searchable_by_order::SearchableByOrder;
pub use trait_sort_order::keys;
pub use trait_sort_order::{SortOrder, AscendingOrder, DescendingOrder, Key, KeyOrder};
pub use trait_sortable::Sortable;
pub use trait_sorted_insert::SortedInsert;
pub use trait_sorted_iterator::SortedIterator;
pub use type_sorted::Sorted;
