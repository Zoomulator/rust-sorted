use super::{SortOrder, SortedIterator};
use std::cmp::Ordering;

/// Adaptor iterator which outputs the intersection of two sorted iterators in
/// linear time. Will ouput the repeated items as many times as they are present in both iterators.
#[derive(Debug)]
pub struct Intersection<I, J>
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

impl<I, J> SortedIterator for Intersection<I, J>
where
    I: SortedIterator,
    I::Ordering: SortOrder<I::Item>,
    J: SortedIterator<Item = I::Item, Ordering = I::Ordering>,
{
    type Ordering = I::Ordering;
}

impl<I, J> Iterator for Intersection<I, J>
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
                    Ordering::Equal => return Some(a),
                    Ordering::Less => self.b = Some(b),
                    Ordering::Greater => self.a = Some(a),
                },
                _ => return None,
            }
        }
    }
}

pub trait IntersectionExt
where
    Self: SortedIterator + Sized,
    Self::Ordering: SortOrder<Self::Item>,
{
    fn intersection<J>(self, J) -> Intersection<Self, J>
    where
        J: SortedIterator<Item = Self::Item, Ordering = Self::Ordering>;
}

impl<I> IntersectionExt for I
where
    I: SortedIterator,
    I::Ordering: SortOrder<I::Item>,
{
    fn intersection<J>(self, j: J) -> Intersection<Self, J>
    where
        J: SortedIterator<Item = Self::Item, Ordering = Self::Ordering>,
    {
        Intersection {
            i: self,
            j,
            a: None,
            b: None,
        }
    }
}
