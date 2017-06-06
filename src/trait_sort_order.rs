use std::cmp::Ordering;

use super::{Sorted, Sortable};

pub trait SortOrder<T> : Clone + Copy {
    fn cmp(&T, &T) -> Ordering;

    fn by_sorting<'a, S: 'a + Sortable<Item=T>>(s: S) -> Sorted<'a, S, Self> {
        Sorted::by_sorting(s)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultOrder;

impl<T> SortOrder<T> for DefaultOrder
where T: Ord + Clone {
    fn cmp(a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }
}