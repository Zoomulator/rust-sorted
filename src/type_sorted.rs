
use std::iter::FromIterator;

use std::ops::Deref;
use std::marker::PhantomData;
use super::{Collection, RetainsOrder, SortOrder, Sortable, SortedInsert, SortedIter,
            SortedIterator, SearchableByOrder};

/// Guarantees that the inner container is sorted in a specific order.
///
/// The [`Sortable`] trait is a requirement for all containers T being wrapped
/// with `Sorted<T>` to enforce certain guarantees for operations that arent
/// strictly read only (i.e creating iterators). See the documentation of
/// [`Sortable`] for details.
/// [`Sortable`]: trait.Sortable.html
#[derive(Debug, PartialEq, Eq)]
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

    pub fn iter(&'a self) -> SortedIter<<&T as IntoIterator>::IntoIter, O>
        where &'a T: IntoIterator
    {
        SortedIter {
            inner: IntoIterator::into_iter(&self.collection),
            ordering: PhantomData,
        }
    }

    // This could unfortunatly not be implemented as the trait From, due to
    // colliding with the blanket impl of From<T> for T.
    // This assumes that the conversion always preserves order.
    // The bound `U: Sortable` mitigates this. You can't convert to say
    // Sorted<BinaryHeap> since it wont implement `Sortable`.
    // It'll be marked as unstable since I believe this gives a weak guarantee
    // and probably should not be added.
    // The Sortable guarantee would not cover all possible conversions between
    // the different collections implementing Sortable. There could possibly be
    // a new trait called SortedFrom that restricts it to known types but there
    // would be n x n implementations of the type.
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

    /// Builds a sorted collection of type T from a sorted iterator.
    ///
    /// As the [`RetainsOrder`] trait is required to be implemented for T, it's
    /// guaranteed that the order of the input iterator is preserved. Since the
    /// iterator must implement SortedIterator, you'll safely get a sorted collection.
    /// [`Sortable`]: trait.Sortable.html
    pub fn from_iter<I>(iter: I) -> Sorted<'a, T, O>
        where T: RetainsOrder + FromIterator<<T as Collection>::Item>,
              I: IntoIterator<Item = T::Item>,
              I::IntoIter: SortedIterator<Ordering = O>,
              <I as IntoIterator>::IntoIter: SortedIterator<Ordering = O>
    {
        Sorted {
            collection: iter.into_iter().collect(),
            ordering: PhantomData,
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
    where T: SearchableByOrder<O>,
          O: SortOrder<T::Item>
{
    pub fn search(&self, a: &T::Item) -> Result<usize, usize> {
        self.collection.search(a)
    }
}

impl<'a, T, O> Sorted<'a, T, O>
    where T: Sortable + SortedInsert<O>,
          O: SortOrder<T::Item>
{
    pub fn insert(&mut self, x: T::Item) {
        self.collection.insert(x)
    }
}

impl<'a, T, O> Sorted<'a, T, O> {
    /// Similar to Option::as_ref. It's mapping the inner type with AsRef.
    pub fn as_ref<U>(&self) -> Sorted<'a, &U, O>
        where T: AsRef<U>,
              U: ?Sized + RetainsOrder
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

impl<'a, T, O> IntoIterator for Sorted<'a, T, O>
    where T: IntoIterator<Item = <T as Collection>::Item> + Sortable
{
    type Item = <T as Collection>::Item;
    type IntoIter = SortedIter<T::IntoIter, O>;
    fn into_iter(self) -> Self::IntoIter {
        SortedIter {
            inner: self.collection.into_iter(),
            ordering: PhantomData,
        }
    }
}

/*
impl<'a, I, T, O> IntoIterator for &'a Sorted<'a, T, O>
    where &'a T: IntoIterator<Item = <I as Iterator>::Item, IntoIter=I>,
          T: Sortable,
          I: Iterator + Sortable
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = SortedIter<<&'a T as IntoIterator>::IntoIter, O>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
*/
