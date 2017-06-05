#[macro_export]
macro_rules! order_by_key {
    ($name:ident: $(fn ($($gen:tt)*) ($entry:ident: $t:ty) -> $r:ty $blk:block)*) => (
        #[derive(Debug, Clone, Copy)]
        struct $name;

        $(impl<$($gen)*> $crate::SortOrder<$t> for $name
        {
            fn sort(s: &mut [$t]) {
                fn key<$($gen)*>($entry: &$t) -> $r $blk
                s.sort_by_key(key)
            }
        })*
    );
}
