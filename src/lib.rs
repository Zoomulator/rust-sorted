#[macro_use]
mod sort_order_macro;
mod sort_order;
mod sorted_iter;
mod sorted_slice;
mod sorted_trait;
mod sorted_vec;


pub use sort_order::{
    SortOrder,
    DefaultOrder
};
pub use sorted_iter::SortedIter;
pub use sorted_slice::SortedSlice;
pub use sorted_trait::Sorted;
pub use sorted_vec::SortedVec;