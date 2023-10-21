use crate::split_suffix::{PartOfSpeech, Suffix};

enum Conjugation {}

struct Node {
    form: String,
    length: usize,
    part_of_speech: PartOfSpeech,
    conjugation: Option<Conjugation>,
    position: usize,
}

struct Word {
    base: String,
    suffixes: Option<Vec<Suffix>>,
    part_of_speech: PartOfSpeech,
    conjugation: Option<Conjugation>,
    position: usize,
    emission_cost: usize,
}

struct Lattice {
    begin_words: Vec<Vec<Word>>,
    end_words: Vec<Vec<Word>>,
}

impl Lattice {
    fn add_word(&mut self, begin_index: usize, end_index: usize, word: Word) {
        self.begin_words[begin_index].push(word.clone());
        self.end_words[end_index].push(word);
    }
}
