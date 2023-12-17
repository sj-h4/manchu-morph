use std::str::FromStr;

use crate::word::{Case, Detail, PartOfSpeech, Word};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FunctionWord {
    pub entry: String,
    pub part_of_speech: PartOfSpeech,
    pub details: Vec<String>,
}

impl FromStr for FunctionWord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let function_words = get_function_word_list();
        for function_word in function_words.iter() {
            if function_word.entry == s {
                return Ok(function_word.clone());
            }
        }
        Err("Not a function word".into())
    }
}

impl Into<Vec<Word>> for FunctionWord {
    // when a function word has some details, create a word for each detail
    fn into(self) -> Vec<Word> {
        let mut words = vec![];

        if self.part_of_speech == PartOfSpeech::Clitic {
            let cases: Result<Vec<Case>, _> = self
                .details
                .iter()
                .map(|case| Case::from_str(case))
                .collect();
            match cases {
                Ok(cases) => {
                    let case_words: Vec<Word> = cases
                        .iter()
                        .map(|case| Word {
                            base: self.entry.clone(),
                            suffixes: None,
                            part_of_speech: PartOfSpeech::Clitic,
                            detail: Some(Detail::Case(case.clone())),
                            emission_cost: -1,
                        })
                        .collect();
                    words.extend(case_words);
                }
                Err(_) => {
                    println!("Invalid case");
                }
            }
        } else {
            // Create a word for each detail
            for detail in self.details.iter() {
                let word = Word {
                    base: self.entry.clone(),
                    suffixes: None,
                    part_of_speech: self.part_of_speech.clone(),
                    detail: Some(Detail::Other(detail.clone())),
                    emission_cost: -1,
                };
                words.push(word);
            }
        }

        words
    }
}

pub fn get_function_word_list() -> Vec<FunctionWord> {
    let data = include_str!("../resources/function_word.json");
    let function_words: Vec<FunctionWord> =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
    function_words
}
