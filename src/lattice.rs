use crate::split_suffix::{PartOfSpeech, Suffix};

enum Conjugation {}

struct Word {
    base: String,
    suffixes: Option<Vec<Suffix>>,
    part_of_speech: PartOfSpeech,
    conjugation: Option<Conjugation>,
    position: usize,
    emission_cost: usize,
}

struct Node {
    words: Vec<Word>,
    emission_cost: usize,
    path_cost: usize,
    left_node: Option<Node>,
}

struct Lattice {
    lattice: Vec<Vec<Node>>,
}

impl Lattice {
    fn add_node(&mut self, node: Node, position: usize) {
        self.lattice[position].push(node);
    }

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
                            + cost_matrix[previous_node.position][current_node.position]
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
