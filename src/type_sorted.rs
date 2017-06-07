#[cfg(feature = "unstable")]
use std::iter::FromIterator;

use std::ops::Deref;
use std::marker::PhantomData;
use std::cmp::Ordering;
use super::{IsSorted, SortOrder, Sortable, SortedInsert, SortedIter};


pub struct Sorted<'a, T: 'a, O: 'a> {
    collection: T,
    ordering: PhantomData<&'a O>,
}

impl<'a, T, O> Sorted<'a, T, O> {
    pub fn as_inner(&self) -> &T {
        &self.collection
    }

    pub fn deref_inner<U>(&'a self) -> Sorted<'a, &U, O>
        where U: ?Sized,
              T: Deref<Target = U>
    {
        Sorted {
            collection: Deref::deref(&self.collection),
            ordering: PhantomData,
        }
    }

    pub fn iter(&'a self) -> SortedIter<<&T as IntoIterator>::IntoIter,O>
        where &'a T: IntoIterator
    {
        SortedIter {
            inner: IntoIterator::into_iter(&self.collection),
            ordering: PhantomData
        }
    }

    // This could unfortunatly not be implemented as the trait From, due to
    // colliding with the blanket impl of From<T> for T.
    // This assumes that the conversion always preserves order.
    // The bound `U: Sortable` mitigates this. You can't convert to say
    // Sorted<BinaryHeap> or something that is guaranteed to be unordered.
    // It'll be marked as unstable since I believe this gives a weak guarantee
    // and might not be added.
    #[cfg(feature = "unstable")]
    pub fn from<'b, U>(sorted: Sorted<U, O>) -> Self
        where T: From<U>,
              U: Sortable
    {
        Sorted {
            collection: From::from(sorted.collection),
            ordering: PhantomData,
        }
    }

    // Again, weak guarantee that FromIterator for T doesn't alter sort order.
    #[cfg(feature = "unstable")]
    pub fn from_iter<I,J>(iter: I) -> Sorted<'a, T, O>
        where I: IntoIterator<IntoIter=SortedIter<J,O>, Item=T::Item>,
              J: Iterator<Item=<T as Sortable>::Item>,
              T: Sortable + FromIterator<<T as Sortable>::Item>,
    {
        Sorted {
            collection: iter.into_iter().collect(),
            ordering: PhantomData
        }
    }
}

impl<'a, T, O> Sorted<'a, T, O>
    where T: Sortable,
          O: SortOrder<T::Item>
{
    pub fn by_sorting(mut collection: T) -> Self {
        collection.sort(O::cmp);
        Self {
            collection,
            ordering: PhantomData,
        }
    }
}

impl<'a, T, O> Sorted<'a, T, O>
    where T: Sortable + SortedInsert<<T as Sortable>::Item>,
          O: SortOrder<T::Item>
{
    pub fn insert(&mut self, x: T::Item) {
        self.collection.insert::<O>(x)
    }
}

impl<'a, T, O> Sorted<'a, T, O>
    where T: Sortable
{
    /// Similar to Option::as_ref. It's mapping the inner type with AsRef.
    pub fn as_ref<U>(&self) -> Sorted<'a, &U, O>
        where T: AsRef<U>,
              U: ?Sized
    {
        Sorted {
            collection: AsRef::as_ref(&self.collection),
            ordering: PhantomData,
        }
    }
}

impl<'a, T, O> Deref for Sorted<'a, T, O> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_inner()
    }
}

impl<'a, T, O> IsSorted for Sorted<'a, T, O> {
    type Ordering = O;
}

impl<'a, T, O> Sortable for Sorted<'a, T, O>
    where T: Sortable
{
    type Item = T::Item;

    fn sort<F>(&mut self, f: F)
        where F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.collection.sort(f)
    }

    fn search<F>(&self, x: &Self::Item, f: F) -> Result<usize, usize>
        where F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        self.collection.search(x, f)
    }
}

impl<'a, T, O> IntoIterator for Sorted<'a, T, O>
    where T: IntoIterator<Item = <T as Sortable>::Item> + Sortable
{
    type Item = <T as Sortable>::Item;
    type IntoIter = SortedIter<T::IntoIter, O>;
    fn into_iter(self) -> Self::IntoIter {
        SortedIter {
            inner: self.collection.into_iter(),
            ordering: PhantomData,
        }
    }
}
