use crate::split_suffix::{PartOfSpeech, Suffix};

enum Conjugation {}

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
struct Node {
    words: Vec<Word>,
    emission_cost: usize,
    /// minimum cost of path from the beginning to the node
    path_cost: usize,
    /// left node of the node in the path with the minimum cost
    left_node: Option<Node>,
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
            let current_nodes = &mut self.lattice[i];
            let previous_nodes = &self.lattice[i - 1];
            for current_node in current_nodes {
                let min_cost_path = previous_nodes
                    .iter().cloned().map(|previous_node| {
                        // TODO: コストを適切に取得する
                        let path_cost = previous_node.path_cost
                            + current_node.emission_cost
                            + cost_matrix[previous_node.category_id][current_node.category_id]
                                .parse::<usize>()
                                .unwrap();
                           (path_cost, previous_node)
                })
                    .min_by_key(|(path_cost, _)| *path_cost);
                if let Some((path_cost, previous_node)) = min_cost_path {
                    current_node.path_cost = path_cost;
                    current_node.left_node = Some(previous_node);
                }
            }
        }
    }
}
