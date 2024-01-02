# Manchu morphological analyzer

README is written in Japanese.

> [!CAUTION]
> `Lattice::get_min_cost_path` は機能しません。

## 使い方

```toml
[dependencies]
manchu-morph = { git = "https://github.com/sj-h4/manchu-morph.git" }
```

```rust
use manchu_morph::lattice::Lattice;

fn main() {
    let sentence = "manju gisun i bithe"

    // 分割・グロスの候補の作成
    let lattice = Lattice::from_sentence(sentence);

    // 満洲文字に変換する場合
    //let mut lattice = Lattice::from_sentence(sentence);
    //lattice.to_manchu_letters();

    // JSON への変換
    let lattice_json = lattice.to_json_string().unwrap();
}
```
