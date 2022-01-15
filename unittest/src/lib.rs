fn add_two(i: i32) -> i32 {
    i + 2
}

mod test {
    use crate::add_two;

    #[test]
    fn add_two_test() {
        assert_eq!(4, add_two(2));
    }
}
