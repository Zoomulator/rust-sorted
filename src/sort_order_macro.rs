#[macro_export]
macro_rules! order_by_key {
    ($name:ident: $(fn ($($gen:tt)*) ($entry:ident: $t:ty) -> $r:ty $blk:block)*) => (
        #[derive(Debug, Clone, Copy)]
        struct $name;

        $(impl<$($gen)*> $crate::SortOrder<$t> for $name
        {
            type Key = $r;

            fn key($entry: &$t) -> $r $blk

            fn sort(s: &mut [$t]) {
                s.sort_by_key(Self::key)
            }

            fn search(s: &[$t], k: &$r) -> Result<usize, usize> {
                s.binary_search_by_key(k, Self::key)
            }
        })*
    );
}
