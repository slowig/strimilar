mod tests {
    use rstest::rstest;
    use strimilar::hamming;

    #[rstest]
    #[case("hello world", "hello world", 0)]
    #[case("hello world", "hello welt!", 4)]
    fn hamming_distance(#[case] left_str: &str, #[case] right_str: &str, #[case] expected_similarity: i32) {
        assert_eq!(hamming(left_str, right_str).unwrap(), expected_similarity);
    }

    #[rstest]
    fn should_return_error_when_strings_have_not_equal_length() {
        match  hamming("string", "string in not equal") {
            Err(err) => assert!(true),
            _ => { assert!(false) }
        }
    }
}