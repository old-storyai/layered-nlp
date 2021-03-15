use crate::ll_line::{LLLine, LLToken, LToken, TextTag};
use crate::type_bucket::AnyAttribute;
use unicode_segmentation::UnicodeSegmentation;

mod get_word_tag;

pub enum InputToken {
    Text {
        /// This text will be split up and TextTags will be added to its pieces
        text: String,
        /// Custom attributes
        attrs: Vec<AnyAttribute>,
    },
    Custom {
        /// Position relative size
        size: usize,
        /// Custom attributes
        attrs: Vec<AnyAttribute>,
    },
}

impl InputToken {
    pub fn text(text: String, attrs: Vec<AnyAttribute>) -> Self {
        InputToken::Text { text, attrs }
    }

    pub fn custom(size: usize, attrs: Vec<AnyAttribute>) -> Self {
        InputToken::Custom { size, attrs }
    }

    pub fn add_attr<T: 'static + std::fmt::Debug>(&mut self, value: T) {
        match self {
            InputToken::Text { attrs, .. } => attrs.push(AnyAttribute::new(value)),
            InputToken::Custom { attrs, .. } => attrs.push(AnyAttribute::new(value)),
        }
    }
}

pub fn create_tokens<F>(input: Vec<InputToken>, get_text_size: F) -> LLLine
where
    F: Fn(&str) -> usize,
{
    let mut start_idx_end_idx_attributes: Vec<(usize, usize, Vec<AnyAttribute>)> = Vec::new();
    let mut lltokens: Vec<LLToken> = Vec::new();
    let mut current_size = 0;
    for (ltokens, attrs) in input.into_iter().map(|input_token| match input_token {
        InputToken::Text { text, attrs } => {
            (create_tokens_for_string(&text, &get_text_size), attrs)
        }
        InputToken::Custom { size, attrs } => (vec![(LToken::Value, size)], attrs),
    }) {
        let from_idx = lltokens.len();
        for (ltoken, size) in ltokens {
            let next_size = current_size + size;
            lltokens.push(LLToken {
                token_idx: lltokens.len(),
                pos_starts_at: current_size,
                pos_ends_at: next_size,
                token: ltoken,
            });

            current_size = next_size;
        }
        let to_idx = lltokens.len() - 1;

        start_idx_end_idx_attributes.push((from_idx, to_idx, attrs));
    }

    let mut ll_line = LLLine::new(lltokens);
    for (start_idx, end_idx, attributes) in start_idx_end_idx_attributes {
        ll_line.add_any_attrs(start_idx, end_idx, attributes);
    }

    ll_line
}

// helper for [create_tokens]
fn create_tokens_for_string<F>(input: &str, get_text_size: F) -> Vec<(LToken, usize)>
where
    F: Fn(&str) -> usize,
{
    // `fold` because we end up splitting more than just unicode word boundaries
    input.split_word_bounds().fold(Vec::new(), |mut ltokens, unicode_word| {
        // Split apart digit word boundaries, because unicode `split_word_bounds` will group digits and commas and points together
        // such as "12,3" and "10.0". We need these to be split up further into ["12", ",", "3"] and ["10", ".", "0"] respectively.
        // http://www.unicode.org/reports/tr29/#Word_Boundaries
        // if \d+[,\.]\d+ or more repeats (3 is minimum)
        if unicode_word.chars().next().unwrap().is_ascii_digit() {
            // make assumtions about the length of every char being 1
            assert!(
                unicode_word.is_ascii(),
                "Unexpected non-ascii digit word boundary: {}",
                unicode_word
            );

            let mut collected_digits = String::new();

            // using a macro since pulling this out into a closure or function would be very verbose
            // as you'd have to pass references to collected_digits, get_text_size, ltokens
            macro_rules! insert_collected_digits {
                () => {
                    // if this triggers, we might just want to check `if collected_digits.len() > 0 { ...`
                    assert!(collected_digits.len() > 0, "Expected length of collected digits to be greater than 0 so we aren't pushing an empty token into the list");
                    let size = get_text_size(&collected_digits);
                    ltokens.push((
                        LToken::Text(std::mem::take(&mut collected_digits), TextTag::NATN),
                        size,
                    ));
                };
            };

            for ch in unicode_word.chars() {
                if ch.is_ascii_digit() {
                    collected_digits.push(ch);
                } else {
                    insert_collected_digits!();

                    let text = String::from(ch);
                    let size = get_text_size(&text);
                    ltokens.push((LToken::Text(text, TextTag::PUNC), size));
                }
            }

            insert_collected_digits!();
        } else {
            ltokens.push((
                LToken::Text(
                    unicode_word.to_string(),
                    get_word_tag::get_unicode_word_tag(unicode_word),
                ),
                get_text_size(unicode_word),
            ));
        }

        ltokens
    })
}

#[cfg(test)]
mod test {
    use super::{create_tokens, InputToken};
    use crate::ll_line::LLLineDisplay;
    use crate::type_bucket::AnyAttribute;

    #[derive(Debug, Clone)]
    enum MarkKind {
        Italic,
        Bold,
    }

    #[derive(Debug, Clone)]
    struct Link {
        href: String,
    }

    #[test]
    fn test_create_tokens() {
        let ll_line = create_tokens(
            vec![
                InputToken::Text {
                    text: String::from("Hello, "),
                    attrs: Vec::new(),
                },
                InputToken::Text {
                    text: String::from("World"),
                    attrs: vec![
                        AnyAttribute::new(MarkKind::Bold),
                        AnyAttribute::new(MarkKind::Italic),
                    ],
                },
                InputToken::Text {
                    text: String::from("!"),
                    attrs: vec![],
                },
            ],
            |text| text.len(),
        );

        let mut ll_line_display = LLLineDisplay::new(&ll_line);
        ll_line_display.include::<MarkKind>();

        insta::assert_display_snapshot!(ll_line_display, @r###"
        Hello  ,     World  !
                     ╰───╯Italic
                     ╰───╯Bold
        "###);
    }

    #[test]
    fn test_create_tokens_email() {
        let ll_line = create_tokens(
            vec![InputToken::Text {
                text: String::from("name@example.com"),
                attrs: vec![
                    AnyAttribute::new(MarkKind::Italic),
                    AnyAttribute::new(Link {
                        href: String::from("mailto:name@example.com"),
                    }),
                ],
            }],
            |text| text.len(),
        );

        // display insta test
        let mut ll_line_display = LLLineDisplay::new(&ll_line);
        ll_line_display.include::<MarkKind>();
        ll_line_display.include::<Link>();

        insta::assert_display_snapshot!(ll_line_display, @r###"
        name  @  example.com
        ╰──────────────────╯Italic
        ╰──────────────────╯Link { href: "mailto:name@example.com" }
        "###);
    }
}
