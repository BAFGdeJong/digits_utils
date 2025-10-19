#[cfg(test)]
mod to_digits_tests {
    use digits_utils::to_digits::ToDigits;

    #[test]
    fn to_digits_i8() {
        let x: i8 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i8 = -123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i8 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
        let x: i8 = -123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_u8() {
        let x: u8 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: u8 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_i16() {
        let x: i16 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i16 = -123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i16 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
        let x: i16 = -123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_u16() {
        let x: u16 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: u16 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_i32() {
        let x: i32 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i32 = -123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i32 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
        let x: i32 = -123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_u32() {
        let x: u32 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: u32 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_i64() {
        let x: i64 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i64 = -123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i64 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
        let x: i64 = -123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_u64() {
        let x: u64 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: u64 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_i128() {
        let x: i128 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i128 = -123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: i128 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
        let x: i128 = -123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_u128() {
        let x: u128 = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: u128 = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

    #[test]
    fn to_digits_usize() {
        let x: usize = 123;
        assert_eq!(x.to_digits(), [1, 2, 3]);
        let x: usize = 123;
        let y: [u8; 2] = x.to_digits();
        assert_eq!(y, [2, 3]);
    }

}

mod to_digits_reversed_tests {
    use digits_utils::to_digits::ToDigits;

    #[test]
    fn to_digits_reversed_i8() {
        let x: i8 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i8 = -123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i8 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
        let x: i8 = -123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_reversed_u8() {
        let x: u8 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: u8 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_reversed_i16() {
        let x: i16 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i16 = -123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i16 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
        let x: i16 = -123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_reversed_u16() {
        let x: u16 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: u16 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_reversed_i32() {
        let x: i32 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i32 = -123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i32 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
        let x: i32 = -123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_reversed_u32() {
        let x: u32 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: u32 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_reversed_i64() {
        let x: i64 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i64 = -123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i64 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
        let x: i64 = -123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_u64() {
        let x: u64 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: u64 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_reversed_i128() {
        let x: i128 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i128 = -123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: i128 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
        let x: i128 = -123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_reversed_u128() {
        let x: u128 = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: u128 = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }

    #[test]
    fn to_digits_reversed_usize() {
        let x: usize = 123;
        assert_eq!(x.to_digits_reversed(), [3, 2, 1]);
        let x: usize = 123;
        let y: [u8; 2] = x.to_digits_reversed();
        assert_eq!(y, [3, 2]);
    }
}