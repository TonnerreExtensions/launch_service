use crate::query::matcher::cleaner::tokenize_and_clean;

pub fn match_query(query: &str, target: &str) -> bool {
    prefix_match(query, target) || initial_match(query, target)
}

/// Simple match that query is the prefix of target
fn prefix_match(query: &str, target: &str) -> bool {
    target.to_lowercase().starts_with(query)
}

/// Complex match that query contains prefixes of target components
fn initial_match(query: &str, target: &str) -> bool {
    let components = tokenize_and_clean(target);
    match_components_prefix(query, &components[..])
}

/// Match query with target components
/// - Example:
///     - `am` matches with `*A*ctivity *M*onitor`
///     - `actmo` matches with `**Act**ivity **Mo**nitor`
fn match_components_prefix(query: &str, target: &[&str]) -> bool {
    if query.is_empty() { true } else if target.is_empty() { false } else {
        let processing = target.first()
            .expect("Get first target component failed");
        for (index, (query_char, target_char)) in query.chars()
            .zip(processing.chars()).enumerate() {
            if query_char.eq_ignore_ascii_case(&target_char) {
                if match_components_prefix(&query[index + 1..], &target[1..]) {
                    return true;
                }
            } else { break; }
        }
        false
    }
}

#[cfg(test)]
mod matcher_test {
    use crate::query::matcher::matcher::{initial_match, prefix_match};

    #[test]
    fn test_prefix_match() {
        let res = prefix_match("saf", "Safari.app");
        assert!(res)
    }

    #[test]
    fn test_initial_match_simple() {
        let res = initial_match("am", "Activity Monitor.app");
        assert!(res)
    }

    #[test]
    fn test_initial_match_complex() {
        let res = initial_match("actmo", "Activity Monitor.app");
        assert!(res)
    }

    #[test]
    fn test_initial_match_incomplete_query() {
        let res = initial_match("am", "Activity Monitor Super.app");
        assert!(res)
    }

    #[test]
    fn test_initial_match_overcomplete_query() {
        let res = initial_match("ams", "Activity Monitor.app");
        assert_eq!(res, false);
    }

    #[test]
    fn test_initial_match_unmatched() {
        let res = initial_match("acx", "Activity Monitor.app");
        assert_eq!(res, false);
    }
}