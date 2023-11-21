use serde::Deserialize;
use strum_macros::EnumString;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuffixRole {
    Functional,
    Derivational,
    Deverbal,
    Denominaladjective,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Conjugation {
    PrefectiveFinite,
    PerfectiveConverb,
    PerfectiveParticle,
    PrefectiveProcessive,
    ImperfectiveFinite,
    ImperfectiveConverb,
    NegativePerfectiveFinite,
    NegativePrefectiveConverb,
    NegativeParticle,
    ProspectiveFinite,
    DesiderativeFinite,
    OptativeFinite,
    DurativeConverb,
    ConditionalConverb,
    ConcessiveConverb,
    TerminativeConverb,
    PrefactoryConverb,
    ApprehensiveConverb,
    SimultaneousConverb,
    AlternativeConverb,
    Denominaladjective,
    PassiveCausativeVerbal,
    Plural,
}

#[derive(Clone, Debug, EnumString, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum Case {
    Nominative,
    Accusative,
    Genitive,
    DativeLocative,
    Instrumental,
    Vocative,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Detail {
    Conjugation(Conjugation),
    Cases(Vec<Case>),
}

/// part of speech which suffix attaches to
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Clitic,
    Unknown,
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
    /// base of the word
    ///
    /// If the word has no suffix, the base is the word itself.
    pub base: String,
    /// suffixes of the word
    ///
    /// The order of suffixes is from the right to the left.
    /// For example, the suffixes of "tuwabumbi" are `vec!["mbi", "bu"]`.
    pub suffixes: Option<Vec<Suffix>>,
    pub part_of_speech: PartOfSpeech,
    /// detail of the word
    ///
    /// If the word is a clitic, the detail is a list of cases
    /// and if the word is a verb, the detail is a conjugation.
    pub detail: Option<Detail>,
    pub emission_cost: usize,
}
