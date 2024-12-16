pub trait CheckedOp {
    fn checked_add(self, rhs: Self) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn checked_mul(self, rhs: Self) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn concat(self, rhs: Self) -> Option<Self>
    where
        Self: std::marker::Sized;
}

macro_rules! checkedop_impl {
    ($integer_type:ty) => {
        impl CheckedOp for $integer_type {
            fn checked_add(self, rhs: Self) -> Option<Self> {
                self.checked_add(rhs)
            }
            fn checked_mul(self, rhs: Self) -> Option<Self> {
                self.checked_mul(rhs)
            }
            fn concat(self, rhs: Self) -> Option<Self> {
                let mut string = self.to_string();
                string.push_str(&rhs.to_string());
                string.parse::<Self>().ok()
            }
        }
    };
}

checkedop_impl!(u8);
checkedop_impl!(u16);
checkedop_impl!(u32);
checkedop_impl!(u64);
checkedop_impl!(u128);
checkedop_impl!(usize);
checkedop_impl!(i8);
checkedop_impl!(i16);
checkedop_impl!(i32);
checkedop_impl!(i64);
checkedop_impl!(i128);
checkedop_impl!(isize);

#[cfg(test)]
mod tests {
    use super::CheckedOp;

    #[test]
    fn interger_concatinate() {
        assert_eq!(1u32.concat(2u32), Some(12u32));
        assert_eq!(56u8.concat(1u8), None);
        assert_eq!(809u32.concat(99u32), Some(80999u32));
    }
}
