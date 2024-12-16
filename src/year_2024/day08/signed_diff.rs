pub trait SignedDiff<T> {
    fn signed_diff(self, other: Self) -> Option<T>;
}

macro_rules! signed_diff_impl {
    ($unsigned:ty, $signed:ty) => {
        impl SignedDiff<$signed> for $unsigned {
            fn signed_diff(self, other: Self) -> Option<$signed> {
                if self < other {
                    <$signed>::try_from(self.abs_diff(other))
                        .ok()
                        .and_then(|i| i.checked_neg())
                } else {
                    <$signed>::try_from(self.abs_diff(other)).ok()
                }
            }
        }
    };
}

signed_diff_impl!(usize, isize);
signed_diff_impl!(u8, i8);
signed_diff_impl!(u16, i16);
signed_diff_impl!(u32, i32);
signed_diff_impl!(u64, i64);
signed_diff_impl!(u128, i128);

#[cfg(test)]
mod tests {
    use crate::year_2024::day08::signed_diff::SignedDiff;

    macro_rules! signed_diff_test {
        ($unsigned:ty, $signed:ty) => {
            assert_eq!(
                <$unsigned>::try_from(8)
                    .unwrap()
                    .signed_diff(<$unsigned>::try_from(9).unwrap()),
                Some(<$signed>::try_from(-1).unwrap())
            );
            assert_eq!(
                <$unsigned>::try_from(9)
                    .unwrap()
                    .signed_diff(<$unsigned>::try_from(8).unwrap()),
                Some(<$signed>::try_from(1).unwrap())
            );
            assert_eq!(
                <$unsigned>::MAX.signed_diff(<$unsigned>::try_from(1).unwrap()),
                None
            );
            assert_eq!(
                <$unsigned>::MAX.signed_diff(<$unsigned>::MAX),
                Some(<$signed>::try_from(0).unwrap())
            );
        };
    }

    #[test]
    fn signed_diff() {
        signed_diff_test!(usize, isize);
        signed_diff_test!(u8, i8);
        signed_diff_test!(u16, i16);
        signed_diff_test!(u32, i32);
        signed_diff_test!(u64, i64);
        signed_diff_test!(u128, i128);
    }
}
