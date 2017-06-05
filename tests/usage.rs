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
fn test_sort_by_first() {
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