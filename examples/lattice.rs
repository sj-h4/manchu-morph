use std::env;

use manchu_morph::lattice::Lattice;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sentence = args[1].clone();
    let mut lattice = Lattice::from_sentence(&sentence);
    lattice.to_manchu_letters();
    let lattice_json = lattice.to_json_string().unwrap();
    println!("{}", lattice_json)
}
