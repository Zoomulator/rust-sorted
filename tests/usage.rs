#[macro_use]
extern crate sorted;

use sorted::*;

order_by_key!{ SortByFirst:
    fn (T: Ord + Clone)(entry: (T,T)) -> T { entry.0.clone() }
}

order_by_key!{ SortBySecond:
    fn (T: Ord + Clone)(entry: (T,T)) -> T { entry.1.clone() }
}


#[test]
fn sort_by_second() {
    let s = vec![(5,3),(2,7),(3,4)];
    let v = SortedVec::by_sorting(s, SortBySecond);
    assert_eq!(
        &[(5,3),(3,4),(2,7)],
        v.as_slice()
    );
}

#[test]
fn slice_from_sorted() {
    let vec = SortedVec::by_sorting(vec![4,9,2,33,1], DefaultOrder);
    let slice = SortedSlice::from(&vec);
    assert_eq!(
        slice,
        &[1,2,4,9,33]
    );
}

#[test]
fn sorted_vec_from_sorted_slice() {
    let mut arr = [5,3,7,9];
    let slice = SortedSlice::by_sorting(&mut arr, DefaultOrder);
    let vec = SortedVec::from(slice);
    assert_eq!(
        vec.as_slice(),
        [3,5,7,9]
    );
}

#[test]
fn take_sorted_iterator() {
    fn take_sorted<I>(sorted: I) where I: IntoIterator<Item=i32> + Sorted {
        let v: Vec<_> = sorted.into_iter().collect();
        assert_eq!(vec![2,3,8,10], v);
    }
    let data: Vec<i32> = vec![3,8,2,10];
    let vec = SortedVec::by_sorting(data, DefaultOrder);
    take_sorted(vec);
}