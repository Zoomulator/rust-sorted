use std::ops::Deref;
use std::marker::PhantomData;
use std::cmp::Ordering;
use super::{
    IsSorted,
    Sortable,
    SortOrder
};


pub struct Sorted<'a,T:'a,O: 'a> {
    collection: T,
    ordering: PhantomData<&'a O>,
}

impl<'a,T,O> Sorted<'a,T,O>
where
    T: Sortable,
    O: SortOrder<T::Item>
{
    pub fn by_sorting(mut collection: T) -> Self {
        collection.sort(O::cmp);
        Self{collection, ordering: PhantomData}
    }
}

impl<'a,T,O> Deref for Sorted<'a,T,O> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}

impl<'a,T,O> IsSorted for Sorted<'a,T,O> {
    type Ordering = O;
}

impl <'a,T,O> Sortable for Sorted<'a,T,O> where T: Sortable {
    type Item = T::Item;

    fn sort<F>(&mut self, f: F)
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering {
        self.collection.sort(f)
    }

    fn search<F>(&self, x: &Self::Item, f: F) -> Result<usize, usize>
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering {
        self.collection.search(x, f)
    }
}