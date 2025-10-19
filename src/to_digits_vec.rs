/// Converts a number into a `Vec<u8>` of its digits.
pub trait ToDigitsVec {
    /// Returns the digits in **normal order** (most significant first) as a `Vec<u8>`.
    /// # Examples
    ///
    /// ```
    /// let x: u8 = 123;
    /// let digits = x.to_digits_vec();
    /// assert_eq!(reversed, vec![2, 5, 5]);
    /// ```
    fn to_digits_vec(&self) -> Vec<u8>;

    /// Returns the digits in **reversed order** (the least significant first) as a `Vec<u8>`.
    /// # Examples
    ///
    /// ```
    /// let x: u8 = 123;
    /// let reversed = x.to_digits_reversed_vec();
    /// assert_eq!(reversed, [5, 2, 2]);
    /// ```
    fn to_digits_reversed_vec(&self) -> Vec<u8>;
}

macro_rules! impl_to_digits_vec_signed {
    ($t:ty) => {
        impl ToDigitsVec for $t {
            #[inline(always)]
            fn to_digits_vec(&self) -> Vec<u8> {
                let mut b = Vec::new();
                let mut v = if self.is_negative() {
                    self.clone().abs()
                } else {
                    self.clone()
                };

                while v > 0 {
                    b.push((v % 10) as u8);
                    v /= 10;
                }

                b.reverse();
                b
            }

            #[inline(always)]
            fn to_digits_reversed_vec(&self) -> Vec<u8> {
                let mut b = Vec::new();
                let mut v = if self.is_negative() {
                    self.clone().abs()
                } else {
                    self.clone()
                };

                while v > 0 {
                    b.push((v % 10) as u8);
                    v /= 10;
                }

                b
            }
        }
    };
}

macro_rules! impl_to_digits_vec_unsigned {
    ($t:ty) => {
        impl ToDigitsVec for $t {
            #[inline(always)]
            fn to_digits_vec(&self) -> Vec<u8> {
                let mut b = Vec::new();
                let mut v = *self;

                while v > 0 {
                    b.push((v % 10) as u8);
                    v /= 10;
                }

                b.reverse();
                b
            }

            #[inline(always)]
            fn to_digits_reversed_vec(&self) -> Vec<u8> {
                let mut b = Vec::new();
                let mut v = *self;

                while v > 0 {
                    b.push((v % 10) as u8);
                    v /= 10;
                }

                b
            }
        }
    };
}

impl_to_digits_vec_signed!(i8);
impl_to_digits_vec_unsigned!(u8);

impl_to_digits_vec_signed!(i16);
impl_to_digits_vec_unsigned!(u16);

impl_to_digits_vec_signed!(i32);
impl_to_digits_vec_unsigned!(u32);

impl_to_digits_vec_signed!(i64);
impl_to_digits_vec_unsigned!(u64);

impl_to_digits_vec_signed!(i128);
impl_to_digits_vec_unsigned!(u128);

impl_to_digits_vec_unsigned!(usize);