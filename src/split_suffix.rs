use core::panic;
use std::error::Error;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SuffixRole {
    Functional,
    Derivational,
    Deverbal,
    Denominaladjective,
}

/// part of speech which suffix attaches to
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PartOfSpeech {
    Noun,
    Verb,
}

#[derive(Clone, Debug, Deserialize)]
struct Suffix {
    /// basic form of suffix
    form: String,
    /// role of suffix
    ///
    /// For example, the role of "mbi" is "functional"
    /// and the role of "bu" is "derivational".
    role: SuffixRole,
    /// part of speech which suffix attaches to
    #[serde(rename = "left_pos")]
    part_of_speech: PartOfSpeech,
}

impl Suffix {
    pub fn new(form: String, role: SuffixRole, part_of_speech: PartOfSpeech) -> Self {
        Self {
            form,
            role,
            part_of_speech,
        }
    }
}

pub struct SplitedWord {
    base: String,
    suffix: Option<Suffix>,
}

impl SplitedWord {
    pub fn new(base: String, suffix: Option<Suffix>) -> Self {
        Self { base, suffix }
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }
}

pub struct FullySplitedWord {
    root: String,
    suffixes: Option<Vec<Suffix>>,
}

fn read_suffix_csv() ->Vec<Suffix> {
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
        Err(err) =>{
            panic!("Error reading suffix csv")
        }
    }
}

/// Spilt a word into a suffix and its base.
///
/// Returns Err if the word is empty or consists entirely of whitespace.
///
/// * `word` - A word to split.
pub fn split_word_into_suffix_base(word: &str) -> Result<SplitedWord, Box<dyn Error>> {
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
            let splited_word = SplitedWord::new(base, Some(suffix));
            return Ok(splited_word);
        }
    }
    Ok(SplitedWord::new(word.to_string(), None))
}

/// Split a word into a suffix and its base recursively until the suffix is not found
/// and return the base and suffixes.
pub fn recurrsive_split(word: &str) -> Result<SplitedWord, Box<dyn Error>> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let valid_word = split_word_into_suffix_base("tuwabumbi");

        let expected_base = "tuwabu";
        let expected_suffix = "mbi";

        match valid_word {
            Ok(splited_word) => {
                assert_eq!(splited_word.base(), expected_base);
                assert_eq!(splited_word.suffix(), Some(expected_suffix));
            }
            Err(_) => assert!(false),
        }

        let whitespace = split_word_into_suffix_base("   ");
        assert!(whitespace.is_err());

        let empty = split_word_into_suffix_base("");
        assert!(empty.is_err());
    }
}
