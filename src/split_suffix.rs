use core::panic;
use std::error::Error;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuffixRole {
    Functional,
    Derivational,
    Deverbal,
    Denominaladjective,
}

/// part of speech which suffix attaches to
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Clitic,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Suffix {
    /// suffix
    ///
    /// For example, "mbi", "ha" or "bu".
    pub(crate) suffix: String,
    /// form of suffix
    pub(crate) form: String,
    /// role of suffix
    ///
    /// For example, the role of "mbi" is "functional"
    /// and the role of "bu" is "derivational".
    pub(crate) role: SuffixRole,
    /// part of speech which suffix attaches to
    #[serde(rename = "left_pos")]
    pub(crate) part_of_speech: PartOfSpeech,
}

pub struct SplitWord {
    base: String,
    /// suffixes of the word
    ///
    /// The order of suffixes is from the right to the left.
    /// For example, the suffixes of "tuwabumbi" are `vec!["mbi", "bu"]`.
    suffixes: Option<Vec<Suffix>>,
}

impl SplitWord {
    pub fn new(base: String, suffixes: Option<Vec<Suffix>>) -> Self {
        Self { base, suffixes }
    }
}

fn read_suffix_csv() -> Vec<Suffix> {
    let rdr = csv::Reader::from_path("resources/suffix.csv");
    match rdr {
        Ok(mut rdr) => {
            let mut suffixes = Vec::new();
            for result in rdr.deserialize() {
                if let Ok(result) = result {
                    let suffix: Suffix = result;
                    suffixes.push(suffix);
                } else {
                    panic!("Validation Error")
                }
            }
            suffixes
        }
        Err(_) => {
            panic!("Error reading suffix csv")
        }
    }
}

/// Spilt a word into a suffix and its base.
///
/// Returns Err if the word is empty or consists entirely of whitespace.
///
/// * `word` - A word to split.
pub fn split_word_into_suffix_base(word: &str) -> Result<SplitWord, Box<dyn Error>> {
    let suffixes = read_suffix_csv();
    if word.is_empty() {
        return Err("Empty string".into());
    }
    if word.chars().all(|c| c.is_whitespace()) {
        return Err("Whitespace string".into());
    }
    for suffix in suffixes.iter() {
        let suffix_form = suffix.form.as_str();
        if word.ends_with(suffix_form) {
            let base = word[..word.len() - suffix_form.len()].to_string();
            let suffix = suffix.clone();
            let suffixes = vec![suffix];
            let splited_word = SplitWord::new(base, Some(suffixes));
            return Ok(splited_word);
        }
    }
    Ok(SplitWord::new(word.to_string(), None))
}

/// Split a word into a suffix and its base recursively until the suffix is not found
/// and return the base and suffixes.
///
/// * `word` - A word to split.
pub fn recursive_split(
    word: &str,
    mut suffixes: Vec<Suffix>,
) -> Result<SplitWord, Box<dyn Error>> {
    let split_word_result: Result<SplitWord, Box<dyn Error>> = split_word_into_suffix_base(word);
    match split_word_result {
        Ok(split_word) => {
            if let Some(suffix) = split_word.suffixes {
                let base = split_word.base;
                suffixes.extend(suffix);
                let split_word = recursive_split(&base, suffixes)?;
                let split_word = SplitWord::new(split_word.base, split_word.suffixes);
                Ok(split_word)
            } else {
                let split_word = SplitWord::new(split_word.base, Some(suffixes));
                Ok(split_word)
            }
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let valid_word = split_word_into_suffix_base("tuwabumbi");

        let expected_base = "tuwabu";
        let expected_suffix = "mbi";

        match valid_word {
            Ok(split_word) => {
                let suffix = split_word.suffixes.unwrap();
                assert_eq!(split_word.base, expected_base);
                assert_eq!(suffix[0].form, expected_suffix);
            }
            Err(_) => assert!(false),
        }

        let whitespace = split_word_into_suffix_base("   ");
        assert!(whitespace.is_err());

        let empty = split_word_into_suffix_base("");
        assert!(empty.is_err());
    }

    #[test]
    fn it_works_recursively() {
        let valid_word = recursive_split("tuwabumbi", vec![]);

        let expected_base = "tuwa";
        let expected_suffix1 = "mbi";
        let expected_suffix2 = "bu";

        match valid_word {
            Ok(split_word) => {
                let suffix = split_word.suffixes.unwrap();
                assert_eq!(split_word.base, expected_base);
                assert_eq!(suffix.len(), 2);
                assert_eq!(suffix[0].form, expected_suffix1);
                assert_eq!(suffix[1].form, expected_suffix2);
            }
            Err(_) => assert!(false),
        }

        let whitespace = recursive_split("   ", vec![]);
        assert!(whitespace.is_err());

        let empty = recursive_split("", vec![]);
        assert!(empty.is_err());
    }
}
