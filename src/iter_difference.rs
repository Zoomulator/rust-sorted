use super::{SortOrder, SortedIterator};
use std::cmp::Ordering;

/// Adaptor iterator which outputs the difference of two sorted iterators in
/// linear time.
#[derive(Debug)]
pub struct Difference<I, J>
where
    I: SortedIterator,
    I::Ordering: SortOrder<I::Item>,
    J: SortedIterator<Item = I::Item, Ordering = I::Ordering>,
{
    i: I,
    j: J,
    a: Option<I::Item>,
    b: Option<J::Item>,
}

impl<I, J> SortedIterator for Difference<I, J>
where
    I: SortedIterator,
    I::Ordering: SortOrder<I::Item>,
    J: SortedIterator<Item = I::Item, Ordering = I::Ordering>,
{
    type Ordering = I::Ordering;
}

impl<I, J> Iterator for Difference<I, J>
where
    I: SortedIterator,
    I::Ordering: SortOrder<I::Item>,
    J: SortedIterator<Item = I::Item, Ordering = I::Ordering>,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let a = self.a.take().or_else(|| self.i.next());
            let b = self.b.take().or_else(|| self.j.next());
            match (a, b) {
                (Some(a), Some(b)) => match I::Ordering::cmp(&a, &b) {
                    Ordering::Less => {
                        self.b = Some(b);
                        return Some(a);
                    }
                    Ordering::Greater => {
                        self.a = Some(a);
                    }
                    Ordering::Equal => {
                        self.b = Some(b);
                    }
                },
                (None, None) => return None,
                (a, None) => return a,
                (None, _) => (),
            }
        }
    }
}

/// Extension class for sorted iterators to provide the difference iterator
/// constructor.
pub trait DifferenceExt
where
    Self: SortedIterator + Sized,
    Self::Ordering: SortOrder<Self::Item>,
{
    fn difference<J>(self, j: J) -> Difference<Self, J>
    where
        J: SortedIterator<Item = Self::Item, Ordering = Self::Ordering>;
}

impl<I> DifferenceExt for I
where
    I: SortedIterator,
    I::Ordering: SortOrder<I::Item>,
{
    fn difference<J>(self, j: J) -> Difference<Self, J>
    where
        J: SortedIterator<Item = Self::Item, Ordering = Self::Ordering>,
    {
        Difference {
            i: self,
            j,
            a: None,
            b: None,
        }
    }
}
