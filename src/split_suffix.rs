use core::panic;
use std::error::Error;

use crate::word::{Detail, PartOfSpeech, Suffix, Word};

/// Spilt a word into a suffix and its base.
///
/// Returns Err if the word is empty or consists entirely of whitespace.
///
/// * `word` - A word to split.
pub fn split_word_into_suffix_base(word: &str) -> Result<Word, String> {
    let suffixes = read_suffix_csv();
    if word.is_empty() {
        return Err("Empty string".into());
    }
    if word.chars().all(|c| c.is_whitespace()) {
        return Err("Whitespace string".into());
    }
    for suffix in suffixes.iter() {
        let suffix_entry = suffix.suffix.as_str();
        if word.ends_with(suffix_entry) {
            let base = word[..word.len() - suffix_entry.len()].to_string();
            let suffix = suffix;
            let suffixes = vec![suffix.clone()];
            let split_word = Word {
                base,
                suffixes: Some(suffixes),
                part_of_speech: suffix.part_of_speech,
                detail: Some(Detail::Conjugation(suffix.conjugation)),
                emission_cost: 0,
            };
            return Ok(split_word);
        }
    }
    Ok(Word {
        base: word.to_string(),
        suffixes: None,
        part_of_speech: PartOfSpeech::Noun,
        detail: None,
        emission_cost: 0,
    })
}

/// Split a word into a suffix and its base recursively until the suffix is not found
/// and return the base and suffixes.
///
/// * `word` - A word to split.
pub fn recursive_split(word: &str, mut suffixes: Vec<Suffix>) -> Result<Word, String> {
    let split_word_result: Result<Word, String> = split_word_into_suffix_base(word);
    match split_word_result {
        Ok(split_word) => {
            if let Some(suffix) = split_word.suffixes {
                let base = split_word.base;
                suffixes.extend(suffix);
                let split_word = recursive_split(&base, suffixes)?;
                let split_word = Word {
                    base: split_word.base,
                    suffixes: split_word.suffixes,
                    part_of_speech: split_word.part_of_speech,
                    detail: split_word.detail,
                    emission_cost: split_word.emission_cost,
                };
                Ok(split_word)
            } else {
                let split_word = Word {
                    base: split_word.base,
                    suffixes: Some(suffixes),
                    part_of_speech: split_word.part_of_speech,
                    detail: split_word.detail,
                    emission_cost: split_word.emission_cost,
                };
                Ok(split_word)
            }
        }
        Err(err) => Err(err.into()),
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
                assert_eq!(suffix[0].suffix, expected_suffix);
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
                assert_eq!(suffix[0].suffix, expected_suffix1);
                assert_eq!(suffix[1].suffix, expected_suffix2);
            }
            Err(_) => assert!(false),
        }

        let whitespace = recursive_split("   ", vec![]);
        assert!(whitespace.is_err());

        let empty = recursive_split("", vec![]);
        assert!(empty.is_err());
    }
}
