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

/// Spilt a word into a suffix and its base.
///
/// Returns Err if the word is empty or consists entirely of whitespace.
pub fn split_word_into_suffix_base(word: &str) -> Result<SplitedWord, String> {
    let suffixes = ["ing", "ed", "s", "ly"];
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
        let valid_word = split_word_into_suffix_base("testly");

        let expected_base = "test";
        let expected_suffix = "ly";

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
