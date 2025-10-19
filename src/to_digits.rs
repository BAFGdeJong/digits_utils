/// Converts a number into an array of its digits.
///
/// `SIZE` specifies the length of the array. Leading zeros are used if the
/// number has fewer digits than `SIZE`.
pub trait ToDigits<const SIZE: usize> {
    /// Returns the digits in **normal order** (most significant first).
    /// # Examples
    ///
    /// ```
    /// let x: u8 = 123;
    /// let digits: [u8; u8::MAX_DIGITS] = x.to_digits();
    /// assert_eq!(digits, [1, 2, 3]);
    ///
    /// let x: u8 = 123;
    /// let digits: [u8; 2] = x.to_digits();
    /// assert_eq!(digits, [2, 3]);
    /// ```
    fn to_digits(&self) -> [u8; SIZE];

    /// Returns the digits in **reversed order** (the least significant first).
    /// # Examples
    ///
    /// ```
    /// let x: u8 = 123;
    /// let reversed: [u8; u8::MAX_DIGITS] = x.to_digits_reversed();
    /// assert_eq!(reversed, [3, 2, 1]);
    ///
    /// let x: u8 = 123;
    /// let reversed: [u8; 2] = x.to_digits_reversed();
    /// assert_eq!(reversed, [3, 2]);
    /// ```
    fn to_digits_reversed(&self) -> [u8; SIZE];
}

macro_rules! impl_to_digits_signed {
    (($($ns:literal),*), $t:ty) => {
        $(
            impl ToDigits<$ns> for $t {
                #[inline(always)]
                fn to_digits(&self) -> [u8; $ns] {
                    let mut b = [0; $ns];

                    #[cfg(feature = "bit_hacks")]
                    let mut v: $t = {
                        let mask = *self >> (<$t>::BITS - 1);
                        (*self ^ mask) - mask
                    };

                    #[cfg(not(feature = "bit_hacks"))]
                    let mut v = if *self < 0 { -(*self) } else { *self };

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

                    #[cfg(feature = "bit_hacks")]
                    let mut v: $t = {
                        let mask = *self >> (<$t>::BITS - 1);
                        (*self ^ mask) - mask
                    };

                    #[cfg(not(feature = "bit_hacks"))]
                    let mut v = if *self < 0 { -(*self) } else { *self };

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

macro_rules! impl_to_digits_unsigned {
    (($($ns:literal),*), $t:ty) => {
        $(
            impl ToDigits<$ns> for $t {
                #[inline(always)]
                fn to_digits(&self) -> [u8; $ns] {
                    let mut b = [0; $ns];
                    let mut v = *self;
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
                    let mut v = *self;
                    let mut i = 0;

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

impl_to_digits_signed!((1, 2, 3), i8);
impl_to_digits_unsigned!((1, 2, 3), u8);

impl_to_digits_signed!((1, 2, 3, 4, 5), i16);
impl_to_digits_unsigned!((1, 2, 3, 4, 5), u16);

impl_to_digits_signed!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10), i32);
impl_to_digits_unsigned!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10), u32);

impl_to_digits_signed!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20), i64);
impl_to_digits_unsigned!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20), u64);

impl_to_digits_signed!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                 31, 32, 33, 34, 35, 36, 37, 38, 39), i128);

impl_to_digits_unsigned!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                 31, 32, 33, 34, 35, 36, 37, 38, 39), u128);

impl_to_digits_unsigned!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20), usize);