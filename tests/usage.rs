#[macro_use]
extern crate sorted;

use sorted::*;

order_by_key!{ Key0AscOrder:
    fn (K: Ord + Copy, T)(entry: (K,T)) -> K { entry.0 }
}

order_by_key!{ KeySecondOrder:
    fn (K: Ord + Copy, T)(entry: (T,K)) -> K { entry.1 }
}

#[test]
fn sorted_array() {
    let arr = [7, 2, 9, 6];
    // Sort the array, resulting in a Sorted<[T;n], AscendingOrder>.
    let v = AscendingOrder::by_sorting(arr);
    // Retrieve a reference to the inner type with .as_inner.
    assert_eq!(v.as_inner(), &[2, 6, 7, 9]);
}

#[test]
fn ascending_order() {
    let arr = AscendingOrder::by_sorting([3, 4, 1, 2]);
    assert_eq!(arr.as_inner(), &[1, 2, 3, 4]);
}

#[test]
fn descending_order() {
    let arr = DescendingOrder::by_sorting([1, 3, 4, 2]);
    assert_eq!(arr.as_inner(), &[4, 3, 2, 1]);
}

#[test]
fn sorted_slice() {
    // You can also refer to a slice that should be sorted.
    // This leaves the ownership outside of the Sorted<> type.
    let mut arr = [3, 2, 4, 1];
    let s = AscendingOrder::by_sorting(&mut arr[..]);
    assert_eq!(s.as_inner(), &[1, 2, 3, 4]);
}

#[test]
fn sorted_vec() {
    let v = AscendingOrder::by_sorting(vec![4, 3, 1, 2]);
    assert_eq!(v.as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn sort_by_first() {
    // It's easy to sort by a tuple key. Specifiy which key and in what order
    // it should be sorted.
    let s = vec![(5, 3), (2, 7), (3, 4)];
    let v = KeyOrder::<keys::Key0, AscendingOrder>::by_sorting(s);
    assert_eq!(&[(2, 7), (3, 4), (5, 3)], v.as_slice());
}

#[test]
fn sort_by_second() {
    let s = vec![(5, 3, 9), (2, 7, 2), (3, 4, 4)];
    let v = KeyOrder::<keys::Key2, DescendingOrder>::by_sorting(s);
    assert_eq!(&[(5, 3, 9), (3, 4, 4), (2, 7, 2)], v.as_slice());
}

#[test]
fn sort_with_property_key_type() {
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Record {
        stuff: i32,
        id: u32,
    };
    impl Record {
        pub fn new(n: u32) -> Self {
            Record { stuff: 0, id: n }
        }
    }

    // Sorting via an arbitrary property takes a bit more work.
    // First define a tag type to identify what property we want to sort by.
    #[derive(Debug, Clone, Copy)]
    struct IdKey;
    // Then implement how to get that key from our Record type.
    impl Key<Record> for IdKey {
        type Key = u32;
        fn key(r: &Record) -> Self::Key {
            r.id
        }
    }

    // You can now sort the Records by a property in any order.
    let v = KeyOrder::<IdKey, AscendingOrder>::by_sorting(vec![Record::new(2),
                                                               Record::new(3),
                                                               Record::new(1)]);
    assert_eq!(&v.as_slice(),
               &[Record::new(1), Record::new(2), Record::new(3)]);
}

#[test]
fn sort_by_property_string() {
    use std::cmp::Ordering;
    // Using strings as keys has some lifetime issues with the previous key
    // pattern. It's however possible, just not as flexible.
    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        x: i32,
    };
    impl Person {
        pub fn new(n: &str) -> Self {
            Self {
                name: n.to_string(),
                x: 0,
            }
        }
    }

    // We'll implement our own SortOrder type. Starting with a tag type.
    #[derive(Debug, Clone, Copy)]
    struct OrderByName;
    // Then implement the SortOrder trait, per struct supporting it.
    impl SortOrder<Person> for OrderByName {
        fn cmp(a: &Person, b: &Person) -> Ordering {
            a.name.cmp(&b.name)
        }
    }

    // It will only provide one ordering for what you define, so if you need to
    // support multiple ways of ordering by name it will result in a bit of
    // boilerplate.
    let v = OrderByName::by_sorting(vec![Person::new("Bob"),
                                         Person::new("Cecil"),
                                         Person::new("Alice")]);
    assert_eq!(v.as_slice(),
               &[Person::new("Alice"),
                 Person::new("Bob"),
                 Person::new("Cecil")]);
}

#[test]
fn sorted_slice_from_sorted_vec() {
    // The as_ref works like Option::as_ref; it returns a Sorted type with the
    // inner type being a reference to the original. If the inner type implements
    // the correct AsRef, it'll work.
    fn take_sorted_slice<'a>(slice: Sorted<'a, &'a [i32], AscendingOrder>) {
        assert_eq!(&[1, 2, 4, 9, 33][..], *slice);
    }
    let vec = AscendingOrder::by_sorting(vec![4, 9, 2, 33, 1]);
    take_sorted_slice(vec.as_ref());
}

#[test]
fn sorted_vec_ref() {
    fn take_sorted_vec<'a>(refvec: Sorted<'a, &'a Vec<i32>, AscendingOrder>) {
        assert_eq!(&[1, 2, 3, 4], refvec.as_slice())
    }
    let vec = AscendingOrder::by_sorting(vec![3, 2, 4, 1]);
    take_sorted_vec(vec.as_ref());
}

#[test]
#[cfg(feature="unstable")]
fn sorted_vec_from_sorted_slice() {
    type SortedVec<'a, T, O> = Sorted<'a, Vec<T>, O>;
    let mut arr = [5, 3, 7, 9];
    let slice = AscendingOrder::by_sorting(&mut arr[..]);
    // This is currently unstable as it doesn't seem possible to guarantee sortedness.
    let vec = SortedVec::from(slice);
    assert_eq!([3, 5, 7, 9], vec.as_slice());
}

#[test]
fn take_sorted_iterator() {
    // Sorted types can generate SortedIterators.
    fn take_sorted<I>(sorted: I)
        where I: IntoIterator<Item = i32>,
              I::IntoIter: SortedIterator<Ordering = AscendingOrder>
    {
        let v: Vec<_> = sorted.into_iter().collect();
        assert_eq!(vec![2, 3, 8, 10], v);
    }
    let vec = AscendingOrder::by_sorting(vec![3, 8, 2, 10]);
    take_sorted(vec);
}

#[test]
fn take_sorted_ref_iterator() {
    // By-ref iterators can only be created via Sorted::iter() right now.
    // I.e there is no IntoIterator for &Sorted<>.
    fn take_sorted_ref<'a, I>(sorted: I)
        where I: IntoIterator<Item = &'a i32>,
              I::IntoIter: SortedIterator
    {
        let v: Vec<_> = sorted.into_iter().cloned().collect();
        assert_eq!([1, 2, 3, 4], v.as_slice());
    }
    let vec = AscendingOrder::by_sorting(vec![3, 4, 1, 2]);
    take_sorted_ref(vec.iter());
}

#[test]
fn sorted_insert() {
    // The Sorted type only provides mutable operations that keep the collection
    // sorted.
    let mut vec = AscendingOrder::by_sorting(vec![4, 8, 2, 0]);
    vec.insert(6);
    assert_eq!([0, 2, 4, 6, 8], vec.as_slice());
}

#[test]
fn sorted_vec_from_sorted_iterator() {
    // You can create Sorted collections from Sorted iterators.
    type SortedVec<'a, T, O> = Sorted<'a, Vec<T>, O>;
    let v0 = AscendingOrder::by_sorting(vec![3, 1, 4, 2]);
    let it = v0.into_iter();
    let v1 = SortedVec::from_iter(it);
    assert_eq!(&[1, 2, 3, 4], v1.as_slice());
}
