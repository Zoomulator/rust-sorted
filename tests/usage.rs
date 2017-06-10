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
    let v = AscendingOrder::by_sorting(arr);
    assert_eq!(*v.as_inner(), [2, 6, 7, 9]);
}

#[test]
fn ascending_order() {
    let arr = AscendingOrder::by_sorting([3, 4, 1, 2]);
    assert_eq!(*arr.as_inner(), [1, 2, 3, 4]);
}

#[test]
fn descending_order() {
    let arr = DescendingOrder::by_sorting([1, 3, 4, 2]);
    assert_eq!(*arr.as_inner(), [4, 3, 2, 1]);
}

#[test]
fn sorted_slice() {
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

fn sort_by_property() {
    struct Person { name: String }
    impl Person {
        pub fn new(n: &str) -> Self { Person{ name: n.to_string() } }
    }
    struct NameKey;
    impl Key<Person> for NameKey {
        type for<'r> Key = &'r str;
        fn key(p: &Person) -> &str {
            &p.name
        }
    }

    let v = KeyOrder::<NameKey, AscendingOrder>::by_sorting(
        vec![Person::new("Bob"), Person::new("Cecil"), Person::new("Alice")]
    );
    assert_eq!(
        v.as_slice(),
        &[Person::new("Alice"), Person::new("Bob"), Person::new("Cecil")]
    );
}

#[test]
fn sorted_slice_from_sorted_vec() {
    fn take_sorted_slice<'a>(slice: Sorted<'a, &'a [i32], AscendingOrder>) {
        assert_eq!(&[1, 2, 4, 9, 33][..], *slice);
    }
    let vec = AscendingOrder::by_sorting(vec![4, 9, 2, 33, 1]);
    take_sorted_slice(vec.as_ref());
}

/// TODO: Can you get a Sorted<'a, &vec, O> from Sorted::as_ref?
#[test]
fn uhm() {
    let vec = AscendingOrder::by_sorting(vec![3,2,4,1]);
    let rvec: Sorted<&Vec<i32>, AscendingOrder> = vec.as_ref();
    rvec.as_slice();
}

#[test]
#[cfg(feature="unstable")]
fn sorted_vec_from_sorted_slice() {
    type SortedVec<'a, T, O> = Sorted<'a, Vec<T>, O>;
    let mut arr = [5, 3, 7, 9];
    let slice = AscendingOrder::by_sorting(&mut arr[..]);
    let vec = SortedVec::from(slice);
    assert_eq!([3, 5, 7, 9], vec.as_slice());
}

#[test]
fn take_sorted_iterator() {
    fn take_sorted<I>(sorted: I)
        where I: IntoIterator<Item = i32> + IsSorted
    {
        let v: Vec<_> = sorted.into_iter().collect();
        assert_eq!(vec![2, 3, 8, 10], v);
    }
    let vec = AscendingOrder::by_sorting(vec![3, 8, 2, 10]);
    take_sorted(vec);
}

#[test]
fn take_sorted_ref_iterator() {
    fn take_sorted_ref<'a,I>(sorted: I)
        where I: IntoIterator<Item = &'a i32> + IsSorted
    {
        let v: Vec<_> = sorted.into_iter().cloned().collect();
        assert_eq!([1,2,3,4], v.as_slice());
    }
    let vec = AscendingOrder::by_sorting(vec![3,4,1,2]);
    take_sorted_ref(vec.iter());
}

#[test]
fn sorted_insert() {
    let mut vec = AscendingOrder::by_sorting(vec![4, 8, 2, 0]);
    vec.insert(6);
    assert_eq!([0, 2, 4, 6, 8], vec.as_slice());
}

#[test]
fn sorted_vec_from_sorted_iterator() {
    type SortedVec<'a, T, O> = Sorted<'a, Vec<T>, O>;
    let v0 = AscendingOrder::by_sorting(vec![3,1,4,2]);
    let it = v0.into_iter();
    let v1 = SortedVec::from_iter(it);
    assert_eq!(
        &[1,2,3,4],
        v1.as_slice()
    );
}