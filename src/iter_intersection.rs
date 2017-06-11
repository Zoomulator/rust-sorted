use std::cmp::Ordering;
use super::{SortOrder, IsSorted};

pub struct Intersection<I,J>
where I: Iterator + IsSorted,
      J: Iterator + IsSorted,
{
    i: I,
    j: J,
    a: Option<I::Item>,
    b: Option<J::Item>
}


impl<I,J> IsSorted for Intersection<I,J>
where I: Iterator + IsSorted,
      I::Ordering: SortOrder<I::Item>,
      J: Iterator<Item=I::Item> + IsSorted<Ordering=I::Ordering>
{
    type Ordering = I::Ordering;
}

impl<I,J> Iterator for Intersection<I,J>
where I: Iterator + IsSorted,
      I::Item: Ord,
      I::Ordering: SortOrder<I::Item>,
      J: Iterator<Item=I::Item> + IsSorted<Ordering=I::Ordering>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let a = self.a.take().or_else(|| self.i.next());
            let b = self.b.take().or_else(|| self.j.next());
            match (a,b) {
                (Some(a), Some(b)) => match I::Ordering::cmp(&a, &b) {
                    Ordering::Equal => {return Some(a)},
                    Ordering::Less => self.b = Some(b),
                    Ordering::Greater => self.a = Some(a),
                },
                _ => return None,
            }
        }
    }
}


pub trait IntersectionExt
where Self: Sized + Iterator + IsSorted
{
    fn intersection<J>(self, J) -> Intersection<Self, J>
    where J: Iterator<Item=Self::Item> + IsSorted<Ordering=Self::Ordering>;
}

impl<I> IntersectionExt for I
where I: Sized + Iterator + IsSorted
{
    fn intersection<J>(self, j: J) -> Intersection<Self, J>
    where J: Iterator<Item=Self::Item> + IsSorted<Ordering=Self::Ordering>
    {
        Intersection { i: self, j, a: None, b: None }
    }
}