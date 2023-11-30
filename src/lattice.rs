use std::vec;

use serde::Serialize;

use crate::{
    edge_cost::get_edge_cost_map,
    function_word::FunctionWord,
    split_clitic::split_word_into_word_clitic,
    split_suffix::generate_all_segmentations,
    word::{Detail, Word},
};

#[derive(Clone, Debug, Serialize)]
pub struct MorphemeNode {
    /// words in the node
    ///
    /// If the token includes a clitic, the clitic is indexed as a word.
    /// For example, "niyalmai" is indexed as `vec!["niyalma", "i"]`.
    words: Vec<Word>,
    emission_cost: isize,
    /// minimum cost of path from the beginning to the node
    path_cost: isize,
    /// left node of the node in the path with the minimum cost
    left_node: Option<Box<MorphemeNode>>,
    /// category id of the node
    ///
    /// The category indicates the part of speech, conjugation, semantic role and so on.
    category: String,
}

impl MorphemeNode {
    fn new(words: Vec<Word>, emission_cost: isize, category: String) -> Self {
        MorphemeNode {
            words,
            emission_cost,
            path_cost: 0,
            left_node: None,
            category,
        }
    }

    /// Create a node from a list of words.
    ///
    /// The argument `words` has multiple words if the token includes a clitic.
    /// If not, the argument `words` has only one word.
    ///
    /// The category of the node depends on the detail of the last word.
    fn from_words(words: Vec<Word>) -> Self {
        let morpheme_node;
        // calculate emmision cost
        let emision_cost = words.iter().map(|word| word.emission_cost).sum::<isize>();
        let last_word = words.last().unwrap();
        let detail = last_word.detail.clone();
        let category;
        match detail {
            Some(detail) => {
                match detail {
                    Detail::Conjugation(conjugation) => {
                        morpheme_node =
                            MorphemeNode::new(words, emision_cost, conjugation.to_string())
                    }
                    Detail::Case(case) => {
                        morpheme_node =
                            MorphemeNode::new(words.clone(), emision_cost, case.to_string())
                    }
                    Detail::Other(other) => {
                        morpheme_node = MorphemeNode::new(words, emision_cost, other);
                    }
                };
            }
            None => {
                category = last_word.part_of_speech.to_string();
                morpheme_node = MorphemeNode::new(words, 0, category);
            }
        }
        morpheme_node
    }
}

/// node separated by a space
///
/// Basically, the node is a word, but it has two words if the word includes a clitic.
///
/// For example, "mini boo" is indexed as `vec!["mini", "boo"]`.
#[derive(Serialize, Clone, Debug)]
struct WordNode(Vec<MorphemeNode>);

impl WordNode {
    fn add_node(&mut self, node: MorphemeNode) {
        self.0.push(node);
    }

    fn add_nodes(&mut self, nodes: Vec<MorphemeNode>) {
        self.0.extend(nodes);
    }

    fn from_token(token: &str) -> Self {
        let mut word_node = WordNode(vec![]);
        let all_segmentations = generate_all_segmentations(token, vec![]);
        for segmentation in all_segmentations {
            let nodes = MorphemeNode::from_words(vec![segmentation]);
            word_node.add_node(nodes);
        }

        // if the token includes a clitic, the clitic is indexed as a word
        if let Ok((word_entry, case_clitics)) = split_word_into_word_clitic(token) {
            let all_segmentations = generate_all_segmentations(word_entry.as_str(), vec![]);
            for segmentation in all_segmentations {
                for case_clitic in case_clitics.iter() {
                    let nodes =
                        MorphemeNode::from_words(vec![segmentation.clone(), case_clitic.clone()]);
                    word_node.add_node(nodes);
                }
            }
        }

        // if the token is a function word, the function word is indexed as a word
        if let Ok(function_word) = token.parse::<FunctionWord>() {
            let words: Vec<Word> = function_word.into();
            let nodes: Vec<MorphemeNode> = words
                .iter()
                .map(|word| MorphemeNode::from_words(vec![word.clone()]))
                .collect();
            word_node.add_nodes(nodes);
        }
        word_node
    }

    fn put_morpheme_node(&mut self, index: usize, morpheme_node: MorphemeNode) {
        self.0[index] = morpheme_node;
    }
}

impl IntoIterator for WordNode {
    type Item = MorphemeNode;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Serialize)]
pub struct Lattice {
    sentence: String,
    lattice: Vec<WordNode>,
}

impl Lattice {
    /// Create a lattice from a sentence.
    pub fn from_sentence(sentence: &str) -> Self {
        let space_separated_token: Vec<&str> = sentence.split_whitespace().collect();
        let mut lattice = Lattice {
            sentence: sentence.to_string(),
            lattice: vec![WordNode(vec![]); space_separated_token.len()],
        };
        for (i, token) in space_separated_token.iter().enumerate() {
            lattice.lattice[i] = WordNode::from_token(token);
        }
        lattice
    }

    /// Serialize a `Lattice` into a JSON string.
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    /// Calculate the minimum cost path from the beginning to the end of the lattice.
    // TODO: コストが同じ場合は一番分割してるやつを選ぶようにする
    pub fn calculate_path_costs(&mut self) {
        let edge_cost_map = get_edge_cost_map();
        for i in 1..self.lattice.len() {
            let previous_word_node = &self.lattice[i - 1].clone();
            let current_word_node = &mut self.lattice[i];
            for (morpheme_node_index, current_node) in
                current_word_node.clone().into_iter().enumerate()
            {
                let min_cost_path = previous_word_node
                    .clone()
                    .into_iter()
                    .map(|previous_node| {
                        let edge_cost = edge_cost_map
                            .get(&(
                                previous_node.category.clone(),
                                current_node.category.clone(),
                            ))
                            .unwrap_or(&0);
                        let path_cost =
                            previous_node.path_cost + current_node.emission_cost + edge_cost;
                        (path_cost, previous_node)
                    })
                    .min_by_key(|(path_cost, _)| *path_cost);
                if let Some((path_cost, previous_node)) = min_cost_path {
                    let new_morpheme_node = MorphemeNode {
                        words: current_node.words.clone(),
                        emission_cost: current_node.emission_cost,
                        path_cost,
                        left_node: Some(Box::from(previous_node)),
                        category: current_node.category.clone(),
                    };
                    current_word_node.put_morpheme_node(morpheme_node_index, new_morpheme_node);
                }
            }
        }
    }

    pub fn get_min_cost_path(&self) -> Vec<Vec<Word>> {
        let mut min_cost_path = vec![];
        let last_word_node = &self.lattice[self.lattice.len() - 1];
        let mut min_cost_node = last_word_node.0[0].clone();
        for word_node in last_word_node.0.iter() {
            if word_node.path_cost < min_cost_node.path_cost {
                min_cost_node = word_node.clone();
            }
        }
        min_cost_path.push(min_cost_node.words.clone());
        let mut left_node = min_cost_node.left_node.clone();
        while let Some(node) = left_node {
            min_cost_path.push(node.words.clone());
            left_node = node.left_node.clone();
        }
        min_cost_path.reverse();
        min_cost_path
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::word::{Conjugation, PartOfSpeech, Suffix, SuffixRole};

    fn create_lattice() -> Lattice {
        // cooha be waki seme tumen cooha be unggifi tosoho. (満文老檔 1 p. 1)
        let sentence = "cooha be waki seme tumen cooha be unggifi tosoho.";
        let word_node_0 = WordNode(vec![
            MorphemeNode {
                words: vec![Word {
                    base: "cooha".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Noun,
                    detail: None,
                    emission_cost: 0,
                }],
                emission_cost: 0,
                path_cost: 0,
                left_node: None,
                category: "".to_string(),
            },
            MorphemeNode {
                words: vec![Word {
                    base: "coo".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "ha".to_string(),
                        conjugation: Conjugation::PerfectiveParticiple,
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    detail: None,
                    emission_cost: 0,
                }],
                emission_cost: 0,
                path_cost: 0,
                left_node: None,
                category: "".to_string(),
            },
        ]);
        let word_node_1 = WordNode(vec![MorphemeNode {
            words: vec![Word {
                base: "be".to_string(),
                suffixes: None,
                part_of_speech: PartOfSpeech::Clitic,
                detail: None,
                emission_cost: 0,
            }],
            emission_cost: 0,
            path_cost: 0,
            left_node: None,
            category: "".to_string(),
        }]);
        let word_node_2 = WordNode(vec![MorphemeNode {
            words: vec![Word {
                base: "waki".to_string(),
                suffixes: None,
                part_of_speech: PartOfSpeech::Noun,
                detail: None,
                emission_cost: 0,
            }],
            emission_cost: 0,
            path_cost: 0,
            left_node: None,
            category: "".to_string(),
        }]);
        let word_node_3 = WordNode(vec![MorphemeNode {
            words: vec![Word {
                base: "seme".to_string(),
                suffixes: None,
                part_of_speech: PartOfSpeech::Noun,
                detail: None,
                emission_cost: 0,
            }],
            emission_cost: 0,
            path_cost: 0,
            left_node: None,
            category: "".to_string(),
        }]);
        let word_node_4 = WordNode(vec![MorphemeNode {
            words: vec![Word {
                base: "tumen".to_string(),
                suffixes: None,
                part_of_speech: PartOfSpeech::Noun,
                detail: None,
                emission_cost: 0,
            }],
            emission_cost: 0,
            path_cost: 0,
            left_node: None,
            category: "".to_string(),
        }]);
        let word_node_5 = WordNode(vec![
            MorphemeNode {
                words: vec![Word {
                    base: "cooha".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Noun,
                    detail: None,
                    emission_cost: 0,
                }],
                emission_cost: 0,
                path_cost: 0,
                left_node: None,
                category: "".to_string(),
            },
            MorphemeNode {
                words: vec![Word {
                    base: "coo".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "ha".to_string(),
                        conjugation: Conjugation::PerfectiveParticiple,
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    detail: None,
                    emission_cost: 0,
                }],
                emission_cost: 0,
                path_cost: 0,
                left_node: None,
                category: "".to_string(),
            },
        ]);
        let word_node_6 = WordNode(vec![MorphemeNode {
            words: vec![Word {
                base: "be".to_string(),
                suffixes: None,
                part_of_speech: PartOfSpeech::Clitic,
                detail: None,
                emission_cost: 0,
            }],
            emission_cost: 0,
            path_cost: 0,
            left_node: None,
            category: "".to_string(),
        }]);
        let word_node_7 = WordNode(vec![
            MorphemeNode {
                words: vec![Word {
                    base: "unggi".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "fi".to_string(),
                        conjugation: Conjugation::PerfectiveConverb,
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    detail: None,
                    emission_cost: 0,
                }],
                emission_cost: 0,
                path_cost: 0,
                left_node: None,
                category: "".to_string(),
            },
            MorphemeNode {
                words: vec![Word {
                    base: "unggi".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "fi".to_string(),
                        conjugation: Conjugation::PerfectiveConverb,
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    detail: None,
                    emission_cost: 0,
                }],
                emission_cost: 0,
                path_cost: 0,
                left_node: None,
                category: "".to_string(),
            },
        ]);
        let word_node_8 = WordNode(vec![
            MorphemeNode {
                words: vec![Word {
                    base: "tosoho".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Noun,
                    detail: None,
                    emission_cost: 0,
                }],
                emission_cost: 0,
                path_cost: 0,
                left_node: None,
                category: 0.to_string(),
            },
            MorphemeNode {
                words: vec![Word {
                    base: "toso".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "ho".to_string(),
                        conjugation: Conjugation::PerfectiveParticiple,
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    detail: None,
                    emission_cost: 0,
                }],
                emission_cost: 0,
                path_cost: 0,
                left_node: None,
                category: "".to_string(),
            },
            MorphemeNode {
                words: vec![Word {
                    base: "to".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "so".to_string(),
                        conjugation: Conjugation::Plural,
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    detail: None,
                    emission_cost: 1,
                }],
                emission_cost: 1,
                path_cost: 0,
                left_node: None,
                category: "".to_string(),
            },
        ]);
        let lattice = Lattice {
            sentence: sentence.to_string(),
            lattice: vec![
                word_node_0,
                word_node_1,
                word_node_2,
                word_node_3,
                word_node_4,
                word_node_5,
                word_node_6,
                word_node_7,
                word_node_8,
            ],
        };
        lattice
    }

    #[test]
    fn it_works() {
        // TODO: まともなテストを書く
        let mut lattice = create_lattice();
        lattice.calculate_path_costs();
        assert_eq!(lattice.lattice[8].0.get(0).unwrap().path_cost, 0);
        assert_eq!(lattice.lattice[8].0.get(1).unwrap().path_cost, 0);
    }

    #[test]
    fn test_word_node_from_token() {
        let word_node = WordNode::from_token("niyalmai");
        let len = word_node.0.len();
        assert_eq!(len, 2);
        assert_eq!(word_node.0[1].words[0].base, "niyalma");
    }

    #[test]
    fn test_lattice_from_sentence() {
        // cooha be waki seme tumen cooha be unggifi tosoho. (満文老檔 1 p. 1)
        let lattice = Lattice::from_sentence("cooha be waki seme tumen cooha be unggifi tosoho.");
        let word_node_cooha = &lattice.lattice[0];
        assert_eq!(word_node_cooha.0[0].words[0].base, "cooha");
        assert_eq!(word_node_cooha.0[1].words[0].base, "coo");
        assert_eq!(
            word_node_cooha.0[1].words[0].suffixes.as_ref().unwrap()[0].suffix,
            "ha"
        );
    }
}
