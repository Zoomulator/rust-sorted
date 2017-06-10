use super::{SortOrder, IsSorted};


pub struct Difference<I,J>
where I: Iterator + IsSorted,
      J: Iterator + IsSorted,
{
    i: I,
    j: J,
    a: Option<I::Item>,
    b: Option<J::Item>,
}

impl<I,J> IsSorted for Difference<I,J>
where I: Iterator + IsSorted,
      I::Ordering: SortOrder<I::Item>,
      J: Iterator<Item=I::Item> + IsSorted<Ordering=I::Ordering>
{
    type Ordering = I::Ordering;
}

impl<I,J> Iterator for Difference<I,J>
where I: Iterator + IsSorted,
      I::Item: Ord,
      I::Ordering: SortOrder<I::Item>,
      J: Iterator<Item=I::Item> + IsSorted<Ordering=I::Ordering>
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let mut n = None;
        while n.is_none() {
            let a = self.a.take().or_else(|| self.i.next());
            let b = self.b.take().or_else(|| self.j.next());
            n = match (a,b) {
                (Some(a), Some(b)) => {
                    if a < b { self.b = Some(b); Some(Some(a)) }
                    else if b < a { self.a = Some(a); None }
                    else { self.b = Some(b); None }
                },
                (None, None) => Some(None),
                (a, None) => Some(a),
                (None, _) => None
            }
        }
        n.and_then(|o|o) // flatten Option<Option<>>
    }
}


pub trait DifferenceExt
where Self: Sized + Iterator + IsSorted
{
    fn difference<J>(self, j: J) -> Difference<Self, J>
    where J: Sized + Iterator<Item=Self::Item> + IsSorted<Ordering=Self::Ordering>;
}

impl<I> DifferenceExt for I
where I: Sized + Iterator + IsSorted
{
    fn difference<J>(self, j: J) -> Difference<Self, J>
    where J: Sized + Iterator<Item=Self::Item> + IsSorted<Ordering=Self::Ordering>
    {
        Difference { i: self, j, a: None, b: None }
    }
}