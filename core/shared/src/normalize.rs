pub fn normalize_query(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn split_keywords(normalized_query: &str) -> Vec<String> {
    normalized_query
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{normalize_query, split_keywords};

    #[test]
    fn normalizes_whitespace_and_case() {
        let input = "  Apple   WATCH 10 ";
        assert_eq!(normalize_query(input), "apple watch 10");
    }

    #[test]
    fn splits_keywords() {
        let query = "apple watch 10";
        assert_eq!(split_keywords(query), vec!["apple", "watch", "10"]);
    }
}
