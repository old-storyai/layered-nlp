use once_cell::sync::Lazy;
use regex::Regex;

use crate::ll_line::TextTag;

// Hand wavy, punctuation is just the characters that can affect how a sentence is split apart
const PUNCTUATION: &[char] = &[',', '.', '!', ';', ':', '?', '\'', '"'];

static IS_WORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\w[\w'-]*$").unwrap());

static IS_SPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+$").unwrap());

pub(crate) fn get_unicode_word_tag(word: &str) -> TextTag {
    if IS_SPACE.is_match(&word) {
        return TextTag::SPACE;
    }

    if word.len() == 1 && PUNCTUATION.contains(&word.chars().next().unwrap()) {
        return TextTag::PUNC;
    }

    if IS_WORD.is_match(&word) {
        return TextTag::WORD;
    }

    TextTag::SYMB
}
