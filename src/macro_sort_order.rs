pub use std::cmp::Ordering;

#[macro_export]
macro_rules! order_by_key {
    ($name:ident: $(fn ($($gen:tt)*) ($entry:ident: $t:ty) -> $r:ty $blk:block)*
    ) => (
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
