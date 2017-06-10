#[macro_use]
pub mod macro_sort_order;

mod sorted_iter;
mod std_slice;
mod std_vec;
mod trait_collection;
mod trait_is_sorted;
mod trait_searchable_by_order;
mod trait_retains_order;
mod trait_sort_order;
mod trait_sortable;
mod trait_sorted_insert;
mod type_sorted;
mod union_iter;

pub use sorted_iter::SortedIter;
pub use trait_collection::Collection;
pub use trait_is_sorted::IsSorted;
pub use trait_searchable_by_order::SearchableByOrder;
pub use trait_retains_order::{RetainsOrder};
pub use trait_sort_order::{SortOrder, AscendingOrder, DescendingOrder, Key,
    KeyOrder
};
pub use trait_sort_order::keys;
pub use trait_sortable::Sortable;
pub use trait_sorted_insert::SortedInsert;
pub use type_sorted::Sorted;
pub use union_iter::UnionExt;