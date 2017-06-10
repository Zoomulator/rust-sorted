use std::cmp::Ordering;
use std::marker::PhantomData;

use super::{Sorted, Sortable};

pub trait SortOrder<T>: Clone + Copy {
    fn cmp(&T, &T) -> Ordering;

    fn by_sorting<'a, S: 'a + Sortable<Item = T>>(s: S) -> Sorted<'a, S, Self> {
        Sorted::by_sorting(s)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AscendingOrder;

impl<T> SortOrder<T> for AscendingOrder
    where T: Ord + Clone
{
    fn cmp(a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DescendingOrder;

impl<T> SortOrder<T> for DescendingOrder
    where T: Ord + Clone
{
    fn cmp(a: &T, b: &T) -> Ordering {
        b.cmp(a)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct KeyOrder<K,O> {
    key: PhantomData<(K,O)>
}

impl<T,K,O> SortOrder<T> for KeyOrder<K,O>
where
    K: Key<T> + Copy,
    O: SortOrder<K::Key>
{
    fn cmp(a: &T, b: &T) -> Ordering {
        O::cmp(&K::key(a), &K::key(b))
    }
}

pub trait Key<T> {
    type Key;
    fn key(&T) -> Self::Key;
}

pub mod keys {
    use super::Key;

    macro_rules! def_tup_key {
        ($keyname:ident; $keynum:tt; $k:ident; $(($($t:tt,)*);)* ) => {
            #[derive(Debug, Clone, Copy)]
            pub struct $keyname;

            $(
                impl<$($t),*> Key<($($t,)*)> for $keyname
                where $k: Copy
                {
                    type Key = $k;
                    fn key(x: &($($t,)*)) -> Self::Key {
                        x.$keynum
                    }
                }
            )*
        }
    }

    def_tup_key! {
        Key0; 0; K;
        (K,);
        (K, T0,);
        (K, T0, T1,);
        (K, T0, T1, T2,);
        (K, T0, T1, T2, T3,);
        (K, T0, T1, T2, T3, T4,);
        (K, T0, T1, T2, T3, T4, T5,);
        (K, T0, T1, T2, T3, T4, T5, T6,);
        (K, T0, T1, T2, T3, T4, T5, T6, T7,);
        (K, T0, T1, T2, T3, T4, T5, T6, T7, T8,);
        (K, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,);
        (K, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,);
        (K, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,);
    }

    def_tup_key! {
        Key1; 1; K;
        (T0, K,);
        (T0, K, T1,);
        (T0, K, T1, T2,);
        (T0, K, T1, T2, T3,);
        (T0, K, T1, T2, T3, T4,);
        (T0, K, T1, T2, T3, T4, T5,);
        (T0, K, T1, T2, T3, T4, T5, T6,);
        (T0, K, T1, T2, T3, T4, T5, T6, T7,);
        (T0, K, T1, T2, T3, T4, T5, T6, T7, T8,);
        (T0, K, T1, T2, T3, T4, T5, T6, T7, T8, T9,);
        (T0, K, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,);
        (T0, K, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,);
    }

    def_tup_key! {
        Key2; 2; K;
        (T0, T1, K,);
        (T0, T1, K, T2,);
        (T0, T1, K, T2, T3,);
        (T0, T1, K, T2, T3, T4,);
        (T0, T1, K, T2, T3, T4, T5,);
        (T0, T1, K, T2, T3, T4, T5, T6,);
        (T0, T1, K, T2, T3, T4, T5, T6, T7,);
        (T0, T1, K, T2, T3, T4, T5, T6, T7, T8,);
        (T0, T1, K, T2, T3, T4, T5, T6, T7, T8, T9,);
        (T0, T1, K, T2, T3, T4, T5, T6, T7, T8, T9, T10,);
        (T0, T1, K, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,);
    }

    def_tup_key! {
        Key3; 3; K;
        (T0, T1, T2, K,);
        (T0, T1, T2, K, T3,);
        (T0, T1, T2, K, T3, T4,);
        (T0, T1, T2, K, T3, T4, T5,);
        (T0, T1, T2, K, T3, T4, T5, T6,);
        (T0, T1, T2, K, T3, T4, T5, T6, T7,);
        (T0, T1, T2, K, T3, T4, T5, T6, T7, T8,);
        (T0, T1, T2, K, T3, T4, T5, T6, T7, T8, T9,);
        (T0, T1, T2, K, T3, T4, T5, T6, T7, T8, T9, T10,);
        (T0, T1, T2, K, T3, T4, T5, T6, T7, T8, T9, T10, T11,);
    }

    def_tup_key! {
        Key4; 4; K;
        (T0, T1, T2, T3, K,);
        (T0, T1, T2, T3, K, T4,);
        (T0, T1, T2, T3, K, T4, T5,);
        (T0, T1, T2, T3, K, T4, T5, T6,);
        (T0, T1, T2, T3, K, T4, T5, T6, T7,);
        (T0, T1, T2, T3, K, T4, T5, T6, T7, T8,);
        (T0, T1, T2, T3, K, T4, T5, T6, T7, T8, T9,);
        (T0, T1, T2, T3, K, T4, T5, T6, T7, T8, T9, T10,);
        (T0, T1, T2, T3, K, T4, T5, T6, T7, T8, T9, T10, T11,);
    }

    def_tup_key! {
        Key5; 5; K;
        (T0, T1, T2, T3, T4, K,);
        (T0, T1, T2, T3, T4, K, T5,);
        (T0, T1, T2, T3, T4, K, T5, T6,);
        (T0, T1, T2, T3, T4, K, T5, T6, T7,);
        (T0, T1, T2, T3, T4, K, T5, T6, T7, T8,);
        (T0, T1, T2, T3, T4, K, T5, T6, T7, T8, T9,);
        (T0, T1, T2, T3, T4, K, T5, T6, T7, T8, T9, T10,);
        (T0, T1, T2, T3, T4, K, T5, T6, T7, T8, T9, T10, T11,);
    }

    def_tup_key! {
        Key6; 6; K;
        (T0, T1, T2, T3, T4, T5, K,);
        (T0, T1, T2, T3, T4, T5, K, T6,);
        (T0, T1, T2, T3, T4, T5, K, T6, T7,);
        (T0, T1, T2, T3, T4, T5, K, T6, T7, T8,);
        (T0, T1, T2, T3, T4, T5, K, T6, T7, T8, T9,);
        (T0, T1, T2, T3, T4, T5, K, T6, T7, T8, T9, T10,);
        (T0, T1, T2, T3, T4, T5, K, T6, T7, T8, T9, T10, T11,);
    }

    def_tup_key! {
        Key7; 7; K;
        (T0, T1, T2, T3, T4, T5, T6, K,);
        (T0, T1, T2, T3, T4, T5, T6, K, T7,);
        (T0, T1, T2, T3, T4, T5, T6, K, T7, T8,);
        (T0, T1, T2, T3, T4, T5, T6, K, T7, T8, T9,);
        (T0, T1, T2, T3, T4, T5, T6, K, T7, T8, T9, T10,);
        (T0, T1, T2, T3, T4, T5, T6, K, T7, T8, T9, T10, T11,);
    }

    def_tup_key! {
        Key8; 8; K;
        (T0, T1, T2, T3, T4, T5, T6, T7, K,);
        (T0, T1, T2, T3, T4, T5, T6, T7, K, T8,);
        (T0, T1, T2, T3, T4, T5, T6, T7, K, T8, T9,);
        (T0, T1, T2, T3, T4, T5, T6, T7, K, T8, T9, T10,);
        (T0, T1, T2, T3, T4, T5, T6, T7, K, T8, T9, T10, T11,);
    }

    def_tup_key! {
        Key9; 9; K;
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, K,);
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, K, T9,);
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, K, T9, T10,);
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, K, T9, T10, T11,);
    }

    def_tup_key! {
        Key10; 10; K;
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, K,);
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, K, T10,);
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, K, T10, T11,);
    }

    def_tup_key! {
        Key11; 11; K;
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, K,);
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, K, T11,);
    }

    def_tup_key! {
        Key12; 12; K;
        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, K,);
    }
}