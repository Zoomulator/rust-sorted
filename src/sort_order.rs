use std::cmp::Ordering;

use super::Sortable;

pub trait SortOrder<T> : Clone + Copy {
    fn cmp(&T, &T) -> Ordering;
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultOrder;

impl<T> SortOrder<T> for DefaultOrder
where T: Ord + Clone {
    fn cmp(a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }
}