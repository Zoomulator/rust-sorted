use super::{Sortable, SortOrder};

pub trait SortedInsert<T>: Sortable {
    fn insert<O>(&mut self, Self::Item) where O: SortOrder<T>;
}
