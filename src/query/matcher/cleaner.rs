use std::collections::HashSet;

pub fn tokenize_and_clean(name: &str) -> Vec<&str> {
    let stop_words: HashSet<&str> = vec!["and", "&", "And"].into_iter().collect();
    tokenize(name)
        .into_iter()
        .filter(|term| !stop_words.contains(term))
        .collect()
}

pub fn tokenize(name: &str) -> Vec<&str> {
    name.split(" ")
        .map(str::trim)
        .filter(|term| !term.is_empty())
        .flat_map(tokenize_camel_case)
        .collect()
}

/// Tokenize camel case to separate components. Supper lower camel and upper camel
fn tokenize_camel_case<'a>(name: &'a str) -> Vec<&'a str> {
    let mut tokens = Vec::<&'a str>::new();
    let mut start_index: usize = 0;
    let first_char = name.chars().next().expect("name is empty");
    let mut prev_case = Case::from(first_char);
    for (index, character) in name.char_indices() {
        let case = Case::from(character);
        let diff = prev_case.diff(&case);
        if (diff == Diff::Sinking && index - start_index > 1)
            || (diff == Diff::CharType)
            || (diff == Diff::Rising)
        {
            tokens.push(&name[start_index..index]);
            start_index = index;
        }
        prev_case = case;
    }
    tokens.push(&name[start_index..]);
    tokens
}

enum Case {
    None,
    Lower,
    Upper,
}

#[derive(Eq, PartialEq)]
enum Diff {
    None,
    Rising,
    Sinking,
    CharType,
}

impl Case {
    fn from(target: char) -> Self {
        match (target.is_alphabetic(), target.is_lowercase()) {
            (false, _) => Case::None,
            (true, true) => Case::Lower,
            (true, false) => Case::Upper,
        }
    }

    fn diff(&self, another: &Case) -> Diff {
        use Case::*;
        match (self, another) {
            (None, None) | (Lower, Lower) | (Upper, Upper) => Diff::None,
            (Lower, Upper) => Diff::Rising,
            (Upper, Lower) => Diff::Sinking,
            _ => Diff::CharType,
        }
    }
}

#[cfg(test)]
mod cleaner_test {
    use crate::query::matcher::cleaner::{tokenize_and_clean, tokenize_camel_case};

    #[test]
    fn test_tokenize_and_clean_simple() {
        let res = tokenize_and_clean("test and abc");
        let expected = vec!["test", "abc"];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tokenize_and_clean_with_camel_case() {
        let res = tokenize_and_clean("test and camelCaseWithIDandUserInfo");
        let expected = vec!["test", "camel", "Case", "With", "ID", "User", "Info"];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tokenize_lower_camel_case() {
        let res = tokenize_camel_case("camelCase");
        let expected = vec!["camel", "Case"];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tokenize_upper_camel_case() {
        let res = tokenize_camel_case("CamelCase");
        let expected = vec!["Camel", "Case"];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tokenize_mixed_camel_case() {
        let res = tokenize_camel_case("IDandUserInformation");
        let expected = vec!["ID", "and", "User", "Information"];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_tokenize_with_non_alphabet() {
        let res = tokenize_camel_case("Go2Shell");
        let expected = vec!["Go", "2", "Shell"];
        assert_eq!(res, expected);
    }
}
