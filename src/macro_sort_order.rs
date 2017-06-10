pub use std::cmp::Ordering;

#[macro_export]
macro_rules! order_by_key {
    ($name:ident: $(fn ($($gen:tt)*) ($entry:ident: $t:ty) -> $r:ty $blk:block)*) => (
        #[derive(Debug, Clone, Copy)]
        struct $name;

        $(impl<$($gen)*> $crate::SortOrder<$t> for $name
        {
            fn cmp(a: &$t, b: &$t) -> $crate::macro_sort_order::Ordering {
                fn key<$($gen)*>($entry: &$t) -> $r $blk
                key(a).cmp(&key(b))
            }
        })*
    );
}

macro_rules! def_order_by {
    (   $name:ident;
        ($a:ident, $b:ident) $cmp:block
        $( ($($generic:tt)*) $t:ty ;)*
    ) => {
        #[derive(Debug, Clone, Copy)]
        struct $name;

        $(
            impl<$($generic)*> $crate::SortOrder<$t> for $name
            {
                fn cmp($a: &$t, $b: &$t) -> $crate::macro_sort_order::Ordering {
                    $cmp
                }
            }
        )*
    }
}

def_order_by!{ Key0AscOrder;
    (a, b) { a.0.cmp(&b.0) }
    (K: Ord + Copy, T0) (K, T0);
    (K: Ord + Copy, T0, T1) (K, T0, T1);
    (K: Ord + Copy, T0, T1, T2) (K, T0, T1, T2);
    (K: Ord + Copy, T0, T1, T2, T3) (K, T0, T1, T2, T3);
    (K: Ord + Copy, T0, T1, T2, T3, T4) (K, T0, T1, T2, T3, T4);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5) (K, T0, T1, T2, T3, T4, T5);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6) (K, T0, T1, T2, T3, T4, T5, T6);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7) (K, T0, T1, T2, T3, T4, T5, T6, T7);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (K, T0, T1, T2, T3, T4, T5, T6, T7, T8);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (K, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (K, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (K, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
}

def_order_by!{ Key1AscOrder;
    (a, b) { a.1.cmp(&b.1) }
    (K: Ord + Copy, T0) (T0, K);
    (K: Ord + Copy, T0, T1) (T0, K, T1);
    (K: Ord + Copy, T0, T1, T2) (T0, K, T1, T2);
    (K: Ord + Copy, T0, T1, T2, T3) (T0, K, T1, T2, T3);
    (K: Ord + Copy, T0, T1, T2, T3, T4) (T0, K, T1, T2, T3, T4);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5) (T0, K, T1, T2, T3, T4, T5);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6) (T0, K, T1, T2, T3, T4, T5, T6);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7) (T0, K, T1, T2, T3, T4, T5, T6, T7);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (T0, K, T1, T2, T3, T4, T5, T6, T7, T8);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, K, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, K, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, K, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
}

def_order_by!{ Key2AscOrder;
    (a, b) { a.2.cmp(&b.2) }
    (K: Ord + Copy, T0, T1) (T0, T1, K);
    (K: Ord + Copy, T0, T1, T2) (T0, T1, K, T2);
    (K: Ord + Copy, T0, T1, T2, T3) (T0, T1, K, T2, T3);
    (K: Ord + Copy, T0, T1, T2, T3, T4) (T0, T1, K, T2, T3, T4);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5) (T0, T1, K, T2, T3, T4, T5);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6) (T0, T1, K, T2, T3, T4, T5, T6);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7) (T0, T1, K, T2, T3, T4, T5, T6, T7);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (T0, T1, K, T2, T3, T4, T5, T6, T7, T8);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, T1, K, T2, T3, T4, T5, T6, T7, T8, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, K, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, K, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
}

def_order_by!{ Key3AscOrder;
    (a, b) { a.3.cmp(&b.3) }
    (K: Ord + Copy, T0, T1, T2) (T0, T1, T2, K);
    (K: Ord + Copy, T0, T1, T2, T3) (T0, T1, T2, K, T3);
    (K: Ord + Copy, T0, T1, T2, T3, T4) (T0, T1, T2, K, T3, T4);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5) (T0, T1, T2, K, T3, T4, T5);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6) (T0, T1, T2, K, T3, T4, T5, T6);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7) (T0, T1, T2, K, T3, T4, T5, T6, T7);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (T0, T1, T2, K, T3, T4, T5, T6, T7, T8);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, T1, T2, K, T3, T4, T5, T6, T7, T8, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, T2, K, T3, T4, T5, T6, T7, T8, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, K, T3, T4, T5, T6, T7, T8, T9, T10, T11);
}

def_order_by!{ Key4AscOrder;
    (a, b) { a.4.cmp(&b.4) }
    (K: Ord + Copy, T0, T1, T2, T3) (T0, T1, T2, T3, K);
    (K: Ord + Copy, T0, T1, T2, T3, T4) (T0, T1, T2, T3, K, T4);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5) (T0, T1, T2, T3, K, T4, T5);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6) (T0, T1, T2, T3, K, T4, T5, T6);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7) (T0, T1, T2, T3, K, T4, T5, T6, T7);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (T0, T1, T2, T3, K, T4, T5, T6, T7, T8);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, T1, T2, T3, K, T4, T5, T6, T7, T8, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, T2, T3, K, T4, T5, T6, T7, T8, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, T3, K, T4, T5, T6, T7, T8, T9, T10, T11);
}

def_order_by!{ Key5AscOrder;
    (a, b) { a.5.cmp(&b.5) }
    (K: Ord + Copy, T0, T1, T2, T3, T4) (T0, T1, T2, T3, T4, K);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5) (T0, T1, T2, T3, T4, K, T5);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6) (T0, T1, T2, T3, T4, K, T5, T6);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7) (T0, T1, T2, T3, T4, K, T5, T6, T7);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (T0, T1, T2, T3, T4, K, T5, T6, T7, T8);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, T1, T2, T3, T4, K, T5, T6, T7, T8, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, T2, T3, T4, K, T5, T6, T7, T8, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, T3, T4, K, T5, T6, T7, T8, T9, T10, T11);
}

def_order_by!{ Key6AscOrder;
    (a, b) { a.6.cmp(&b.6) }
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5) (T0, T1, T2, T3, T4, T5, K);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6) (T0, T1, T2, T3, T4, T5, K, T6);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7) (T0, T1, T2, T3, T4, T5, K, T6, T7);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (T0, T1, T2, T3, T4, T5, K, T6, T7, T8);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, T1, T2, T3, T4, T5, K, T6, T7, T8, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, T2, T3, T4, T5, K, T6, T7, T8, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, T3, T4, T5, K, T6, T7, T8, T9, T10, T11);
}

def_order_by!{ Key7AscOrder;
    (a, b) { a.7.cmp(&b.7) }
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6) (T0, T1, T2, T3, T4, T5, T6, K);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7) (T0, T1, T2, T3, T4, T5, T6, K, T7);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (T0, T1, T2, T3, T4, T5, T6, K, T7, T8);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, T1, T2, T3, T4, T5, T6, K, T7, T8, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, T2, T3, T4, T5, T6, K, T7, T8, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, T3, T4, T5, T6, K, T7, T8, T9, T10, T11);
}

def_order_by!{ Key8AscOrder;
    (a, b) { a.8.cmp(&b.8) }
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7) (T0, T1, T2, T3, T4, T5, T6, T7, K);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (T0, T1, T2, T3, T4, T5, T6, T7, K, T8);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, T1, T2, T3, T4, T5, T6, T7, K, T8, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, T2, T3, T4, T5, T6, T7, K, T8, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, T3, T4, T5, T6, T7, K, T8, T9, T10, T11);
}

def_order_by!{ Key9AscOrder;
    (a, b) { a.9.cmp(&b.9) }
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8) (T0, T1, T2, T3, T4, T5, T6, T7, T8, K);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, T1, T2, T3, T4, T5, T6, T7, T8, K, T9);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, T2, T3, T4, T5, T6, T7, T8, K, T9, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, T3, T4, T5, T6, T7, T8, K, T9, T10, T11);
}

def_order_by!{ Key10AscOrder;
    (a, b) { a.10.cmp(&b.10) }
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, K);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, K, T10);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, K, T10, T11);
}

def_order_by!{ Key11AscOrder;
    (a, b) { a.11.cmp(&b.11) }
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, K);
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, K, T11);
}

def_order_by!{ Key12AscOrder;
    (a, b) { a.12.cmp(&b.12) }
    (K: Ord + Copy, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, K);
}