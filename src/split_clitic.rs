use std::str::FromStr;

use crate::function_word::{get_function_word_list, FunctionWord};
use crate::word::{Case, Detail, PartOfSpeech, Word};

struct CaseClitic {
    entry: String,
    cases: Vec<Case>,
}

impl TryFrom<FunctionWord> for CaseClitic {
    type Error = String;
    /// create a list of case clitics by extracting case clitics from a list of function words
    ///
    /// If a clitic has some cases, `CaseClitic` from it has them.
    fn try_from(function_word: FunctionWord) -> Result<Self, Self::Error> {
        if function_word.part_of_speech != PartOfSpeech::Clitic {
            return Err("Not a clitic".into());
        }

        let cases: Result<Vec<Case>, _> = function_word
            .details
            .iter()
            .map(|case| Case::from_str(case))
            .collect();
        match cases {
            Ok(cases) => Ok(CaseClitic {
                entry: function_word.entry,
                cases,
            }),
            Err(_) => Err("Invalid case".into()),
        }
    }
}

impl Into<Word> for CaseClitic {
    fn into(self) -> Word {
        Word {
            base: self.entry,
            suffixes: None,
            part_of_speech: PartOfSpeech::Clitic,
            detail: Some(Detail::Cases(self.cases)),
            emission_cost: 0,
        }
    }
}

/// Split a word into a word and a clitic.
///
/// The word is not be fully split into a word and suffixes;
/// the field `suffixes` of the returned `Word` is `None`.
pub fn split_word_into_word_clitic(word: &str) -> Result<Vec<Word>, String> {
    let function_words = get_function_word_list();
    if word.is_empty() {
        return Err("Empty string".into());
    }
    if word.chars().all(|c| c.is_whitespace()) {
        return Err("Whitespace string".into());
    }
    for function_word in function_words.iter() {
        let Ok(case_clitic) = CaseClitic::try_from(function_word.clone()) else {
            continue;
        };

        let clitic_entry = case_clitic.entry.as_str();
        if word.ends_with(clitic_entry) {
            let base = word[..word.len() - clitic_entry.len()].to_string();
            let split_words = vec![
                Word {
                    base,
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Unknown,
                    detail: None,
                    emission_cost: 0,
                },
                case_clitic.into(),
            ];
            return Ok(split_words);
        }
    }
    Ok(vec![Word {
        base: word.to_string(),
        suffixes: None,
        part_of_speech: PartOfSpeech::Unknown,
        detail: None,
        emission_cost: 0,
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_word_into_word_clitic() {
        let word = "niyalmai";
        let split_words = split_word_into_word_clitic(word).unwrap();
        assert_eq!(split_words.len(), 2);
        assert_eq!(split_words[0].base, "niyalma");
        assert_eq!(split_words[1].base, "i");
        assert_eq!(
            split_words[1].detail,
            Some(Detail::Cases(vec![Case::Genitive, Case::Nominative]))
        );
    }
}
