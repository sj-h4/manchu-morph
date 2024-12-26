# Manchu morphological analyzer

> [!CAUTION]
> This is a work in progress.
> `Lattice::get_min_cost_path` doesn't work.

> [!NOTE]
> This tool create a lattice from a sentence and outputs a JSON string.

## Usage

```toml
[dependencies]
manchu-morph = { git = "https://github.com/sj-h4/manchu-morph.git" }
```

```rust
use manchu_morph::lattice::Lattice;

fn main() {
    let sentence = "manju gisun i bithe"

    // Split the sentence into morphemes and create a lattice
    let lattice = Lattice::from_sentence(sentence);

    // If you want to see the lattice in Manchu letters:
    //let mut lattice = Lattice::from_sentence(sentence);
    //lattice.to_manchu_letters();

    // Convert the lattice to a JSON string
    let lattice_json = lattice.to_json_string().unwrap();
}
```
