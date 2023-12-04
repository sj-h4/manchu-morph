use manchu_converter::ManchuConverter;
use unicode_segmentation::UnicodeSegmentation;

/// Checks if a token is a valid phoneme structure.
///
/// The patterns for valid Manchu syllables are:
///  1. V, CV, VV, CVV
///  2. VC, CVC, VVC, CVVC
///
/// Generally, (C)V(V)(C) is a valid Manchu syllable.
///
/// The last consonant of a syllable can be b, t, k, m, n, ng, l, r or s.
/// The second vowel in VV can be i or o.
///
/// (津曲 (2002) 『満洲語入門20講』)
pub fn is_valid_structure(token: &str) -> bool {
    let result = token.convert_to_manchu();
    let Ok(manchu_token) = result else {
        return false;
    };
    let graphemes = manchu_token.graphemes(true).collect::<Vec<&str>>();
    let len = graphemes.len();
    if len == 0 {
        return false;
    }
    // 1文字の場合
    if len == 1 {
        return is_vowel(graphemes[0]);
    }
    // 2文字以上かつ、語末に子音が連続して現れる場合
    else if is_consonant(graphemes[len - 1]) && is_consonant(graphemes[len - 2]) {
        return false;
    }
    // 2文字以上かつ、語末に子音が現れる場合
    else if is_consonant(graphemes[len - 1]) {
        return is_valid_last_consonant(graphemes[len - 1]);
    } else {
        return true;
    }
}

pub fn is_unusual_final_consonant(token: &str) -> bool {
    let result = token.convert_to_manchu();
    let Ok(manchu_token) = result else {
        return false;
    };
    let graphemes = manchu_token.graphemes(true).collect::<Vec<&str>>();
    let len = graphemes.len();
    if len == 0 {
        return false;
    }
    let unusual_final_consonants = ["b", "t", "k", "m", "ng", "l", "r", "s"]
        .iter()
        .map(|x| x.convert_to_manchu().unwrap())
        .collect::<Vec<String>>();
    unusual_final_consonants.contains(&graphemes[len - 1].to_string())
}

fn is_vowel(char: &str) -> bool {
    let vowels = ["a", "e", "i", "o", "u", "v"]
        .iter()
        .map(|x| x.convert_to_manchu().unwrap())
        .collect::<Vec<String>>();
    vowels.contains(&char.to_string())
}

fn is_consonant(char: &str) -> bool {
    let consonants: Vec<String> = [
        "n", "ng", "b", "p", "s", "š", "x", "k", "g", "h", "l", "m", "t", "d", "r", "j", "y", "c",
        "f", "w", "ts'", "dz", "k'", "g'", "h'", "c'y",
    ]
    .iter()
    .map(|x| x.convert_to_manchu().unwrap())
    .collect();
    consonants.contains(&char.to_string())
}

fn is_valid_last_consonant(char: &str) -> bool {
    let last_consonants = ["b", "t", "k", "m", "n", "ng", "l", "r", "s"]
        .iter()
        .map(|x| x.convert_to_manchu().unwrap())
        .collect::<Vec<String>>();
    last_consonants.contains(&char.to_string())
}
