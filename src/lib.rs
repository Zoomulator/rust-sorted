#[macro_use]
pub mod macro_sort_order;

mod sorted_iter;
mod trait_is_sorted;
mod trait_sort_order;
mod trait_sortable;
mod trait_sorted_insert;
mod type_sorted;

pub use sorted_iter::SortedIter;
pub use trait_is_sorted::IsSorted;
pub use trait_sort_order::{SortOrder, AscendingOrder, DescendingOrder};
pub use trait_sortable::Sortable;
pub use trait_sorted_insert::SortedInsert;
pub use type_sorted::Sorted;
