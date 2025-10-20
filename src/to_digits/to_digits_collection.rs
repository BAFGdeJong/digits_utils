use super::ToDigitsCollection;

use std::iter::FromIterator;

macro_rules! impl_to_digits_collection {
    ($t:ty, $signed:tt) => {
        impl ToDigitsCollection for $t {
            #[inline(always)]
            fn to_digits_into_vec(&self) -> Vec<u8> {
                let mut b = Vec::new();
                let mut v = abs_assigner!($signed, *self, $t);

                while v > 0 {
                    b.push((v % 10) as u8);
                    v /= 10;
                }

                b.reverse();
                b
            }

            #[inline(always)]
            fn to_digits_into<T>(&self) -> T where T: FromIterator<u8> {
                let mut b = Vec::new();
                let mut v = abs_assigner!($signed, *self, $t);

                while v > 0 {
                    b.push((v % 10) as u8);
                    v /= 10;
                }

                b.reverse();
                T::from_iter(b)
            }

            #[inline(always)]
            fn to_digits_reversed_into_vec(&self) -> Vec<u8>{
                let mut b = Vec::new();
                let mut v = abs_assigner!($signed, *self, $t);

                while v > 0 {
                    b.push((v % 10) as u8);
                    v /= 10;
                }

                b
            }

            #[inline(always)]
            fn to_digits_reversed_into<T>(&self) -> T where T: FromIterator<u8> {
                let mut b = Vec::new();
                let mut v = abs_assigner!($signed, *self, $t);

                while v > 0 {
                    b.push((v % 10) as u8);
                    v /= 10;
                }

                T::from_iter(b)
            }
        }
    };
}

#[cfg(feature = "enable_8_bit")]
impl_to_digits_collection!(i8, true);
#[cfg(feature = "enable_8_bit")]
impl_to_digits_collection!(u8, false);

#[cfg(feature = "enable_16_bit")]
impl_to_digits_collection!(i16, true);
#[cfg(feature = "enable_16_bit")]
impl_to_digits_collection!(u16, false);

#[cfg(feature = "enable_32_bit")]
impl_to_digits_collection!(i32, true);
#[cfg(feature = "enable_32_bit")]
impl_to_digits_collection!(u32, false);

#[cfg(feature = "enable_64_bit")]
impl_to_digits_collection!(i64, true);
#[cfg(feature = "enable_64_bit")]
impl_to_digits_collection!(u64, false);

#[cfg(feature = "enable_128_bit")]
impl_to_digits_collection!(i128, true);
#[cfg(feature = "enable_128_bit")]
impl_to_digits_collection!(u128, false);

impl_to_digits_collection!(isize, true);
impl_to_digits_collection!(usize, false);