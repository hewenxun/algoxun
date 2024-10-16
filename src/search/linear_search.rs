pub fn linear_search<T: Eq>(items: &[T], target: &T) -> Option<usize> {
    for (index, item) in items.iter().enumerate() {
        if *item == *target {
            return Some(index)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_integer() {
        let integers = vec![1, 2, 3, 4, 5];
        assert_eq!(linear_search(&integers, &1), Some(0));
        assert_eq!(linear_search(&integers, &5), Some(4));
        assert_eq!(linear_search(&integers, &6), None);
    }

    #[test]
    fn search_char() {
        let chars = vec!['a', 'b', 'c', '🤪', '💨'];
        assert_eq!(linear_search(&chars, &'a'), Some(0));
        assert_eq!(linear_search(&chars, &'💨'), Some(4));
        assert_eq!(linear_search(&chars, &'d'), None);
    }

    #[test]
    fn search_str() {
        let strs = vec!["a", "b", "c", "d", "e"];
        assert_eq!(linear_search(&strs, &"a"), Some(0));
        assert_eq!(linear_search(&strs, &"e"), Some(4));
        assert_eq!(linear_search(&strs, &"f"), None);
    }

    #[test]
    fn search_string() {
        let strings = vec![String::from("a"), String::from("b"), String::from("c")];
        assert_eq!(linear_search(&strings, &"a".to_string()), Some(0));
        assert_eq!(linear_search(&strings, &"c".to_string()), Some(2));
        assert_eq!(linear_search(&strings, &"d".to_string()), None);
    }
}