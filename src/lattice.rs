use serde::Serialize;

use crate::{
    split_clitic::split_word_into_word_clitic, split_suffix::generate_all_segmentations, word::Word,
};

#[derive(Clone, Debug, Serialize)]
struct MorphemeNode {
    /// words in the node
    ///
    /// If the token includes a clitic, the clitic is indexed as a word.
    /// For example, "niyalmai" is indexed as `vec!["niyalma", "i"]`.
    words: Vec<Word>,
    emission_cost: usize,
    /// minimum cost of path from the beginning to the node
    path_cost: usize,
    /// left node of the node in the path with the minimum cost
    left_node: Option<Box<MorphemeNode>>,
    /// category id of the node
    ///
    /// The category indicates the part of speech, conjugation, semantic role and so on.
    category_id: usize,
}

impl From<Word> for MorphemeNode {
    fn from(word: Word) -> Self {
        MorphemeNode {
            words: vec![word],
            emission_cost: 0,
            path_cost: 0,
            left_node: None,
            category_id: 0,
        }
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
    fn from_token(token: &str) -> Self {
        let mut word_node = WordNode(vec![]);
        let all_segmentations = generate_all_segmentations(token, vec![]);
        for segmentation in all_segmentations {
            let node = MorphemeNode::from(segmentation);
            word_node.add(node);
        }

        let words = split_word_into_word_clitic(token).expect("Cannot split word");
        if words.len() == 2 {
            let word_entry = words[0].base.as_str();
            let all_segmentations = generate_all_segmentations(word_entry, vec![]);
            for segmentation in all_segmentations {
                let node = MorphemeNode {
                    words: vec![segmentation, words[1].clone()],
                    emission_cost: 0,
                    path_cost: 0,
                    left_node: None,
                    category_id: 0,
                };
                word_node.add(node);
            }
        }
        word_node
    }

    fn add(&mut self, node: MorphemeNode) {
        self.0.push(node);
    }

    fn get(&self, index: usize) -> Option<&MorphemeNode> {
        self.0.get(index)
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
    pub fn calculate_path_costs(&mut self, cost_matrix: Vec<Vec<String>>) {
        for i in 1..self.lattice.len() {
            let previous_nodes = &self.lattice[i - 1].clone();
            let current_nodes = &mut self.lattice[i];
            for mut current_node in current_nodes.clone().into_iter() {
                let min_cost_path = previous_nodes
                    .clone()
                    .into_iter()
                    .map(|previous_node| {
                        // TODO: コストを適切に取得する
                        // default cost is 0
                        //let edge_cost = cost_matrix[previous_node.category_id][current_node.category_id]
                        //    .parse::<usize>()
                        //    .unwrap_or(0);
                        let edge_cost = 0;
                        let path_cost =
                            previous_node.path_cost + current_node.emission_cost + edge_cost;
                        (path_cost, previous_node)
                    })
                    .min_by_key(|(path_cost, _)| *path_cost);
                if let Some((path_cost, previous_node)) = min_cost_path {
                    current_node.path_cost = path_cost;
                    current_node.left_node = Some(Box::from(previous_node));
                }
            }
        }
    }

    fn add_node(&mut self, node: MorphemeNode, position: usize) {
        self.lattice[position].add(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word::{Conjugation, PartOfSpeech, Suffix, SuffixRole};

    #[test]
    fn it_works() {
        // cooha be waki seme tumen cooha be unggifi tosoho. (満文老檔 1 p. 1)
        let mut lattice = Lattice {
            sentence: "cooha be waki seme tumen cooha be unggifi tosoho.".to_string(),
            lattice: vec![WordNode(vec![]); 11],
        };
        lattice.add_node(
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
                category_id: 0,
            },
            0,
        );
        lattice.add_node(
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
                category_id: 0,
            },
            0,
        );
        lattice.add_node(
            MorphemeNode {
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
                category_id: 0,
            },
            1,
        );
        lattice.add_node(
            MorphemeNode {
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
                category_id: 0,
            },
            2,
        );
        lattice.add_node(
            MorphemeNode {
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
                category_id: 0,
            },
            3,
        );
        lattice.add_node(
            MorphemeNode {
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
                category_id: 0,
            },
            4,
        );
        lattice.add_node(
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
                category_id: 0,
            },
            5,
        );
        lattice.add_node(
            MorphemeNode {
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
                category_id: 0,
            },
            6,
        );
        lattice.add_node(
            MorphemeNode {
                words: vec![Word {
                    base: "unggifi".to_string(),
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
                category_id: 0,
            },
            7,
        );
        lattice.add_node(
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
                category_id: 0,
            },
            8,
        );
        lattice.add_node(
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
                category_id: 0,
            },
            8,
        );
        lattice.calculate_path_costs(vec![vec![]]);
        assert_eq!(lattice.lattice[8].get(0).unwrap().path_cost, 0);
        assert_eq!(lattice.lattice[8].get(1).unwrap().path_cost, 1);
    }
}
