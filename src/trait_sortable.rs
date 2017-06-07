use std::cmp::Ordering;

pub trait Sortable {
    type Item;

    fn sort<F>(&mut self, f: F) where F: FnMut(&Self::Item, &Self::Item) -> Ordering;

    fn search<F>(&self, &Self::Item, F) -> Result<usize, usize>
        where F: FnMut(&Self::Item, &Self::Item) -> Ordering;
}
