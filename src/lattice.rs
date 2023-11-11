use crate::split_suffix::{PartOfSpeech, Suffix};

#[derive(Clone, Debug)]
enum Conjugation {}

#[derive(Clone, Debug)]
struct Word {
    base: String,
    suffixes: Option<Vec<Suffix>>,
    part_of_speech: PartOfSpeech,
    /// conjugation of the word
    ///
    /// For example, the conjugation of "tuwame" is "converb" and the conjugation of "tuwahe" is "perfective".
    conjugation: Option<Conjugation>,
    emission_cost: usize,
}

/// node of lattice
///
/// The `Node` is an unit separated by a space in the input sentence.
#[derive(Clone, Debug)]
struct Node {
    words: Vec<Word>,
    emission_cost: usize,
    /// minimum cost of path from the beginning to the node
    path_cost: usize,
    /// left node of the node in the path with the minimum cost
    left_node: Option<Box<Node>>,
    /// category id of the node
    ///
    /// The category indicates the part of speech, conjugation, semantic role and so on.
    category_id: usize,
}

struct Lattice {
    lattice: Vec<Vec<Node>>,
}

impl Lattice {
    fn add_node(&mut self, node: Node, position: usize) {
        self.lattice[position].push(node);
    }

    /// calculate the minimum cost path from the beginning to the end of the lattice
    fn calculate_path_costs(&mut self, cost_matrix: Vec<Vec<String>>) {
        for i in 1..self.lattice.len() {
            let previous_nodes = &self.lattice[i - 1].clone();
            let current_nodes = &mut self.lattice[i];
            for current_node in current_nodes {
                let min_cost_path = previous_nodes
                    .iter()
                    .cloned()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::split_suffix::SuffixRole;

    #[test]
    fn it_works() {
        // cooha be waki seme tumen cooha be unggifi tosoho. (満文老檔 1 p. 1)
        let mut lattice = Lattice {
            lattice: vec![vec![]; 11],
        };
        lattice.add_node(
            Node {
                words: vec![Word {
                    base: "cooha".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Noun,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "coo".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "ha".to_string(),
                        form: "perfective participle".to_string(),
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "be".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Clitic,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "waki".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Noun,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "seme".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Noun,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "tumen".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Noun,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "cooha".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Noun,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "be".to_string(),
                    suffixes: None,
                    part_of_speech: PartOfSpeech::Clitic,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "unggifi".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "fi".to_string(),
                        form: "perfective converb".to_string(),
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "toso".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "ho".to_string(),
                        form: "perfective participle".to_string(),
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    conjugation: None,
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
            Node {
                words: vec![Word {
                    base: "to".to_string(),
                    suffixes: Some(vec![Suffix {
                        suffix: "so".to_string(),
                        form: "pulural".to_string(),
                        role: SuffixRole::Functional,
                        part_of_speech: PartOfSpeech::Noun,
                    }]),
                    part_of_speech: PartOfSpeech::Noun,
                    conjugation: None,
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
        assert_eq!(lattice.lattice[8][0].path_cost, 0);
        assert_eq!(lattice.lattice[8][1].path_cost, 1);
    }
}
