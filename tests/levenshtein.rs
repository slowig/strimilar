mod tests {
    use rstest::rstest;
    use strimilar::levenshtein;

    #[rstest]
    #[case("hello", "hello", 0)]
    #[case("worlds", "worxyz", 3)]
    #[case("hello worlds", "helo welt", 5)]
    #[case("", "", 0)]
    #[case("", "asd", 3)]
    fn levenshtein_similarity(#[case] left_str: &str, #[case] right_str: &str, #[case] expected_similarity: i32) {
        assert_eq!(levenshtein(left_str, right_str), expected_similarity)
    }
}