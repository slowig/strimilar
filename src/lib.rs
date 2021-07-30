//Returns a distance calculated by Levenshtein method
//
//```
//let distance = levenshtein("hello", "world");
//
//assert_eq!(distance, 5);
//```
pub fn levenshtein(left_string: &str, right_string: &str) -> i32 {
    let mut matrix = vec![vec![0; right_string.len()+1]; left_string.len()+1];

    if left_string.len() == 0 || right_string.len() == 0 {
        return usize::max(left_string.len(), right_string.len()) as i32;
    }

    for (j_index, j_char) in right_string.chars().enumerate() {
        let j_index = j_index + 1;
        for (i_index, i_char) in left_string.chars().enumerate() {
            let i_index = i_index + 1;
            let mut cost = 0;
            if i_char != j_char {
                cost = 1;
            }
            matrix[i_index][j_index] = i32::min(
                matrix[i_index-1][j_index] + 1,
                i32::min(
                    matrix[i_index][j_index-1] + 1,
                    matrix[i_index-1][j_index-1] + cost
                )
            )
        }
    }

    return matrix[left_string.len()][right_string.len()];
}