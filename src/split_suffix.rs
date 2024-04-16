use core::panic;

use crate::{
    phoneme::is_valid_structure,
    word::{Detail, PartOfSpeech, Suffix, Word},
};

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
            // Skip if the base is not a valid phoneme structure.
            if !is_valid_structure(&base) {
                continue;
            }

            let suffixes = vec![suffix.clone()];
            let split_word = Word::new(
                base,
                Some(suffixes),
                suffix.part_of_speech,
                Some(Detail::Conjugation(suffix.conjugation)),
            );
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

/// Generate all possible segmentations of a word.
pub fn generate_all_segmentations(token: &str, mut words: Vec<Word>) -> Vec<Word> {
    // First, add the original word to the list of words.
    if words.len() == 0 {
        words.push(Word {
            base: token.to_string(),
            suffixes: None,
            part_of_speech: PartOfSpeech::Noun,
            detail: None,
            emission_cost: 0,
        });
        words.push(Word {
            base: token.to_string(),
            suffixes: None,
            part_of_speech: PartOfSpeech::Adverb,
            detail: None,
            emission_cost: 0,
        });
    }
    let segmented_word = split_word_into_suffix_base(token).expect("Error splitting word");
    match segmented_word.suffixes {
        Some(suffixes) => {
            let mut suffixes = suffixes.clone();
            let previous_suffix = words[words.len() - 1].suffixes.clone().unwrap_or(vec![]);
            suffixes.extend(previous_suffix);
            let conjugation = suffixes.last().unwrap().conjugation.clone();
            let part_of_speech = suffixes.last().unwrap().part_of_speech.clone();

            let segmented_word = Word::new(
                segmented_word.base,
                Some(suffixes),
                part_of_speech,
                Some(Detail::Conjugation(conjugation)),
            );

            words.push(segmented_word.clone());
            let base = segmented_word.base;
            generate_all_segmentations(&base, words)
        }
        None => words,
    }
}

fn read_suffix_csv() -> Vec<Suffix> {
    let csv = include_str!("../resources/suffix.csv");
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let mut suffixes = Vec::new();
    for result in rdr.deserialize() {
        if let Ok(result) = result {
            let suffix: Suffix = result;
            suffixes.push(suffix);
        } else {
            panic!("{}", result.unwrap_err());
        }
    }
    suffixes
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
    fn test_generate_all_segmentations() {
        let valid_word = generate_all_segmentations("tuwabumbi", vec![]);
        assert_eq!(valid_word.len(), 4);
        assert_eq!(valid_word[0].base, "tuwabumbi");
    }
}
