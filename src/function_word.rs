use std::fs;

use crate::word::PartOfSpeech;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FunctionWord {
    pub entry: String,
    pub part_of_speech: PartOfSpeech,
    pub details: Vec<String>,
}

pub fn get_function_word_list() -> Vec<FunctionWord> {
    let data = fs::read_to_string("resources/function_word.json").expect("Unable to read file");
    let function_words: Vec<FunctionWord> =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
    function_words
}
