pub fn split_string(split: &str, string: String) -> Vec<String> {
    if string.is_empty() {
        vec!()
    } else {
        string.split(split).map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        let result = split_string(" ", String::from("happy new year"));
        assert_eq!(result, vec!("happy".to_string(), "new".to_string(), "year".to_string()));
    }

    #[test]
    fn test_split_string_with_empty_input() {
        let result = split_string(" ", String::from(""));
        assert_eq!(result, Vec::<String>::new());
    }
}
