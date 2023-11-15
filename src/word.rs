use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuffixRole {
    Functional,
    Derivational,
    Deverbal,
    Denominaladjective,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Conjugation {
    PerfectiveConverb,
    PerfectiveParticle,
    Plural,
}

/// part of speech which suffix attaches to
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Clitic,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Suffix {
    /// suffix
    ///
    /// For example, "mbi", "ha" or "bu".
    pub suffix: String,
    /// conjugation of suffix of the word which the suffix attaches to
    #[serde(rename = "form")]
    pub conjugation: Conjugation,
    /// role of suffix
    ///
    /// For example, the role of "mbi" is "functional"
    /// and the role of "bu" is "derivational".
    pub role: SuffixRole,
    /// part of speech of the word which the suffix attaches to
    #[serde(rename = "left_pos")]
    pub part_of_speech: PartOfSpeech,
}

#[derive(Clone, Debug)]
pub struct Word {
    pub base: String,
    /// suffixes of the word
    ///
    /// The order of suffixes is from the right to the left.
    /// For example, the suffixes of "tuwabumbi" are `vec!["mbi", "bu"]`.
    pub suffixes: Option<Vec<Suffix>>,
    pub part_of_speech: PartOfSpeech,
    /// conjugation of the word
    ///
    /// For example, the conjugation of "tuwame" is "converb" and the conjugation of "tuwahe" is "perfective".
    pub conjugation: Option<Conjugation>,
    pub emission_cost: usize,
}
