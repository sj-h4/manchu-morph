use std::str::FromStr;
use strum_macros::EnumString;

use crate::function_word::FunctionWord;

#[derive(EnumString)]
pub enum Case {
    Nominative,
    Accusative,
    Genitive,
    Dative,
    Instrumental,
    Locative,
    Vocative,
}

pub struct CaseClitic {
    entry: String,
    cases: Vec<Case>,
}

impl TryFrom<FunctionWord> for CaseClitic {
    type Error = String;
    /// create a list of case clitics by extracting case clitics from a list of function words
    fn try_from(function_word: FunctionWord) -> Result<Self, Self::Error> {
        if function_word.part_of_speech != "clitic" {
            return Err("Not a clitic".into());
        }

        let cases: Result<Vec<Case>, _> = function_word
            .details
            .iter()
            .map(|case| Case::from_str(case))
            .collect();
        match cases {
            Ok(cases) => Ok(CaseClitic {
                entry: function_word.entry,
                cases,
            }),
            Err(_) => Err("Invalid case".into()),
        }
    }
}
