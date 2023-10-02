pub struct SplitedWord {
    base: String,
    suffix: Option<String>,
}

impl SplitedWord {
    pub fn new(base: String, suffix: Option<String>) -> Self {
        Self { base, suffix }
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }
}

enum SuffixRole {
    Functional,
    Derivational,
    Deverbal,
    Denominaladjective,
}

enum POS {
    Noun,
    Verb,
}

struct Suffix {
    form: String,
    role: SuffixRole,
    part_of_speech: POS,
}

impl Suffix {
    pub fn new(form: String, role: SuffixRole, part_of_speech: POS) -> Self {
        Self { form, role, part_of_speech }
    }
}

// TODO: 読み込んだCSVを構造体に流し込む
fn read_suffix_csv() -> Result<Vec<String>, String> {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("resources/suffix.csv");
    match rdr {
        Ok(mut rdr) => {
            let mut suffixes = Vec::new();
            for result in rdr.records() {
                match result {
                    Ok(record) => {
                        let suffix = record.get(0).unwrap().to_string();
                        suffixes.push(suffix);
                    }
                    Err(_) => return Err("Error reading suffix.csv".to_string()),
                }
            }
            Ok(suffixes)
        }
        Err(err) => Err(err.to_string()),
    }
}

/// Spilt a word into a suffix and its base.
///
/// Returns Err if the word is empty or consists entirely of whitespace.
///
/// * `word` - A word to split.
pub fn split_word_into_suffix_base(word: &str) -> Result<SplitedWord, String> {
    let suffixes = read_suffix_csv();
    let suffixes = match suffixes {
        Ok(suffixes) => suffixes,
        Err(err) => {
            println!("Error reading suffix.csv: {}", err);
            return Err(err);
        }
    };
    if word.is_empty() {
        return Err("Empty string".to_string());
    }
    if word.chars().all(|c| c.is_whitespace()) {
        return Err("Whitespace string".to_string());
    }
    for suffix in suffixes.iter() {
        if word.ends_with(suffix) {
            let base = word[..word.len() - suffix.len()].to_string();
            let suffix = suffix.to_string();
            let splited_word = SplitedWord::new(base, Some(suffix));
            return Ok(splited_word);
        }
    }
    Ok(SplitedWord::new(word.to_string(), None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let valid_word = split_word_into_suffix_base("tuwabumbi");

        let expected_base = "tuwabu";
        let expected_suffix = "mbi";

        match valid_word {
            Ok(splited_word) => {
                assert_eq!(splited_word.base(), expected_base);
                assert_eq!(splited_word.suffix(), Some(expected_suffix));
            }
            Err(_) => assert!(false),
        }

        let whitespace = split_word_into_suffix_base("   ");
        assert!(whitespace.is_err());

        let empty = split_word_into_suffix_base("");
        assert!(empty.is_err());
    }
}
