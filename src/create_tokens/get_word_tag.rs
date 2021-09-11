use crate::ll_line::TextTag;

// Hand wavy, punctuation is just the characters that can affect how a sentence is split apart
const PUNCTUATION: &[char] = &[',', '.', '!', ';', ':', '?', '\'', '"'];

pub(crate) fn get_unicode_word_tag(word: &str) -> TextTag {
    if is_spaces(&word) {
        TextTag::SPACE
    } else if word.len() == 1 && PUNCTUATION.contains(&word.chars().next().unwrap()) {
        TextTag::PUNC
    } else if is_word(&word) {
        TextTag::WORD
    } else {
        TextTag::SYMB
    }
}

fn is_spaces(input: &str) -> bool {
    for c in input.chars() {
        if !c.is_whitespace() {
            return false;
        }
    }
    return true;
}

/// We count "don't", "peoples'", and "baseball-card" to be words!
fn is_word(input: &str) -> bool {
    let mut cs = input.chars();
    match cs.next() {
        Some(first_c) if first_c.is_alphabetic() => {
            // continue...
        }
        Some(_ /* nonalphabetic first char */) | None => return false, // non-alpha or empty
    }

    for c in cs {
        // can be an alphabetic
        if c.is_alphabetic() {
            continue;
        } else {
            // can be a dash or apostrophe
            match c {
                '\'' | '-' | '–' => continue,
                _ => {
                    return false;
                }
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_word() {
        assert!(super::is_word("don't"));
        assert!(super::is_word("A"));
        assert!(super::is_word("Story"));
        assert!(super::is_word("cameras'"));
        // not
        assert!(!super::is_word(""), "empty is not word");
        assert!(!super::is_word("😀"));
        assert!(!super::is_word("🦶🏽"));
        assert!(!super::is_word("$"));
        assert!(!super::is_word("'"));
        assert!(!super::is_word("%'"));
        assert!(!super::is_word("$12"));
        assert!(!super::is_word("'tis"));
    }

    #[test]
    fn test_is_spaces() {
        // https://jkorpela.fi/chars/spaces.html
        assert!(super::is_spaces(" "));
        assert!(
            super::is_spaces("  \u{00A0}"),
            "spaces with non-break space"
        );
        assert!(super::is_spaces(" \t"));
        assert!(super::is_spaces("\t\t"));
        // not
        assert!(!super::is_word(""), "empty is not spaces");
        assert!(!super::is_spaces("don't"));
        assert!(!super::is_spaces("A"));
        assert!(!super::is_spaces("Story"));
        assert!(!super::is_spaces("cameras'"));
        assert!(!super::is_spaces("😀"));
        assert!(!super::is_spaces("🦶🏽"));
        assert!(!super::is_spaces("$"));
        assert!(!super::is_spaces("'"));
        assert!(!super::is_spaces("%'"));
        assert!(!super::is_spaces("$12"));
        assert!(!super::is_spaces("'tis"));
    }
}
