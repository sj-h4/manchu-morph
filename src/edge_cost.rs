use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EdgeCost {
    left_category: String,
    right_category: String,
    cost: isize,
}

impl EdgeCost {
    fn vec_load() -> Vec<EdgeCost> {
        let cost_path = "resources/edge_cost.json";
        let data = fs::read_to_string(cost_path).expect("Unable to read file");
        let edge_costs: Vec<EdgeCost> =
            serde_json::from_str(&data).expect("JSON was not well-formatted");
        edge_costs
    }
}

pub fn get_edge_cost_map() -> HashMap<(String, String), isize> {
    let edge_costs = EdgeCost::vec_load();
    let mut edge_cost_map = HashMap::new();
    for edge_cost in edge_costs {
        let left_category_id = edge_cost.left_category;
        let right_category_id = edge_cost.right_category;
        let cost = edge_cost.cost;
        edge_cost_map.insert((left_category_id, right_category_id), cost);
    }
    edge_cost_map
}
