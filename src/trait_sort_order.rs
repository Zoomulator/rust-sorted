use std::cmp::Ordering;

use super::{Sorted, Sortable};

pub trait SortOrder<T> : Clone + Copy {
    fn cmp(&T, &T) -> Ordering;

    fn by_sorting<'a, S: 'a + Sortable<Item=T>>(s: S) -> Sorted<'a, S, Self> {
        Sorted::by_sorting(s)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AscendingOrder;

impl<T> SortOrder<T> for AscendingOrder
where T: Ord + Clone {
    fn cmp(a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DescendingOrder;

impl<T> SortOrder<T> for DescendingOrder
where T: Ord + Clone {
    fn cmp(a: &T, b: &T) -> Ordering {
        b.cmp(a)
    }
}