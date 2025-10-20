#[cfg(test)]
mod to_digits_tests {
    use std::collections::LinkedList;
    use digits_utils::to_digits::ToDigitsCollection;

    #[test]
    fn test_to_digits_collection() {
        let x: i8 = 123;
        let mut y = LinkedList::<u8>::new();
        let one: u8 = 1;
        let two: u8 = 2;
        let three: u8 = 3;
        y.push_back(one);
        y.push_back(two);
        y.push_back(three);
        let z = x.to_digits_into::<LinkedList<u8>>();
        assert_eq!(z, y);
    }

}