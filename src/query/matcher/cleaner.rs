use std::collections::HashSet;

lazy_static! {
    pub static ref STOP_WORDS: HashSet<&'static str> = vec!["and", "&", "And"].into_iter().collect();
}

pub fn tokenize_and_clean(name: &str) -> Vec<&str> {
    tokenize(name)
        .into_iter()
        .filter(|term| STOP_WORDS.contains(term))
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
    let mut prev_is_capital: u8 = name.chars().next()
        .expect("name is empty")
        .is_uppercase() as u8;
    let is_case_different = |prev: u8, curr: u8| prev ^ curr == 1;
    for (index, character) in name.char_indices() {
        if is_case_different(prev_is_capital, character.is_uppercase() as u8) &&
            index - start_index > 1 {
            tokens.push(&name[start_index..index]);
            start_index = index;
        }
        prev_is_capital = character.is_uppercase() as u8;
    }
    tokens.push(&name[start_index..]);
    tokens
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
}