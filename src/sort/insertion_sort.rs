pub fn insertion_sort<T: Ord>(items: &mut [T]) {
    if items.len() < 2 {
        return
    }

    let last_idx = items.len() - 1;
    for i in 0..last_idx {
        let mut j = i + 1;
        while j > 0 && items[j] < items[j - 1] {
            items.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_integers() {
        let mut integers = vec![4, 2, 5, 3, 1];
        insertion_sort(&mut integers);
        assert_eq!(integers, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_chars() {
        let mut chars = vec!['b', 'c', 'a', 'd', '😔'];
        insertion_sort(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', '😔']);
    }

    #[test]
    fn sort_strs() {
        let mut strs = vec!["bc", "ab", "cd", "ef", "de"];
        insertion_sort(&mut strs);
        assert_eq!(strs, ["ab", "bc", "cd", "de", "ef"]);
    }

    #[test]
    fn sort_strings() {
        let mut strings = vec![String::from("bc"), String::from("ab"), String::from("cd"), String::from("ef"), String::from("de")];
        insertion_sort(&mut strings);
        assert_eq!(strings, ["ab".to_string(), "bc".to_string(), "cd".to_string(), "de".to_string(), "ef".to_string()]);
    }
}