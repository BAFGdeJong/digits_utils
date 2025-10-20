use super::ToDigits;

macro_rules! impl_to_digits{
    (($($ns:literal),*), $t:ty, $signed:tt) => {
        $(
            impl ToDigits<$ns> for $t {
                #[inline(always)]
                fn to_digits(&self) -> [u8; $ns] {
                    let mut b = [0; $ns];
                    let mut v = abs_assigner!($signed, *self, $t);

                    let mut i = $ns;

                    while v > 0 && i > 0 {
                        i -= 1;
                        b[i] = (v % 10) as u8;
                        v /= 10;
                    }

                    b
                }

                #[inline(always)]
                fn to_digits_reversed(&self) -> [u8; $ns] {
                    let mut b = [0; $ns];
                    let mut v = abs_assigner!($signed, *self, $t);

                    let mut i= 0;

                    while v > 0 && i < $ns {
                        b[i] = (v % 10) as u8;
                        v /= 10;
                        i += 1;
                    }

                    b
                }
            }
        )*
    };
}

#[cfg(feature = "enable_8_bit")]
impl_to_digits!((1, 2, 3), i8, true);
#[cfg(feature = "enable_8_bit")]
impl_to_digits!((1, 2, 3), u8, false);

#[cfg(feature = "enable_16_bit")]
impl_to_digits!((1, 2, 3, 4, 5), i16, true);
#[cfg(feature = "enable_16_bit")]
impl_to_digits!((1, 2, 3, 4, 5), u16, false);

#[cfg(feature = "enable_32_bit")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10), i32, true);
#[cfg(feature = "enable_32_bit")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10), u32, false);

#[cfg(feature = "enable_64_bit")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20), i64, true);
#[cfg(feature = "enable_64_bit")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20), u64, false);

#[cfg(feature = "enable_128_bit")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                 31, 32, 33, 34, 35, 36, 37, 38, 39), i128, true);
#[cfg(feature = "enable_128_bit")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                 31, 32, 33, 34, 35, 36, 37, 38, 39), u128, false);

#[cfg(feature = "usize8")]
impl_to_digits!((1, 2, 3), isize, true);
#[cfg(feature = "usize8")]
impl_to_digits!((1, 2, 3), usize, false);

#[cfg(feature = "usize16")]
impl_to_digits!((1, 2, 3, 4, 5), isize, true);
#[cfg(feature = "usize16")]
impl_to_digits!((1, 2, 3, 4, 5), usize, false);

#[cfg(feature = "usize32")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10), isize, true);
#[cfg(feature = "usize32")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10), isize, false);

#[cfg(feature = "usize64")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20), isize, true);
#[cfg(feature = "usize64")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20), usize, false);

#[cfg(feature = "usize128")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                 31, 32, 33, 34, 35, 36, 37, 38, 39), isize, true);
#[cfg(feature = "usize128")]
impl_to_digits!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                 31, 32, 33, 34, 35, 36, 37, 38, 39), usize, false);