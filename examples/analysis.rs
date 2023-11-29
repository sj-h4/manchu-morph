use manchu_morph::lattice::Lattice;

fn main() {
    let mut lattice = Lattice::from_sentence("cooha be waki seme tumen cooha be unggifi tosoho.");
    lattice.calculate_path_costs();
    let min_cost_path = lattice.get_min_cost_path();
    let json_str = serde_json::to_string(&min_cost_path).unwrap();
    println!("{}", json_str);
}
