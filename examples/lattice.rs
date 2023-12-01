use manchu_morph::lattice::Lattice;

fn main() {
    let sentence = "g'aldan dorji wesimbuhe bithe i jise";
    let lattice = Lattice::from_sentence(sentence);
    let lattice_json = lattice.to_json_string().unwrap();
    println!("{}", lattice_json)
}
