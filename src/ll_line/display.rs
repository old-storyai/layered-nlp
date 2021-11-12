use super::*;
use unicode_width::UnicodeWidthStr;

pub struct LLLineDisplay<'a> {
    ll_line: &'a LLLine,
    include_attrs: Vec<(LRange, String)>,
}

// 0,  1,     2,   3, - LRange indexes
// 0,  1,     5,   6, - LLToken::pos_starts_at indexes
// 1,  5,     6,   8, - LLToken::pos_ends_at indexes
// $   1000   .    00
//                ╰NATN
//            ╰PUNC
//     ╰NATN
// ╰PUNC
//     ╰────────────╯ Amount()
// ╰────────────────╯ Money($, Num)
//
// 0,  1,     2,   3, - LRange indexes
// 0,  1,     5,   6, - LLToken::pos_starts_at indexes
// 1,  5,     6,   8, - LLToken::pos_ends_at indexes
// _   1000   .    00    ;    123
//                            ╰NATN
//                       ╰PUNC
//                 ╰NATN
//            ╰PUNC
//     ╰NATN
// ╰SPACE
//     ╰────────────╯ Amount(..)
//                            ╰─╯ Amount(..)
impl<'a> std::fmt::Display for LLLineDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SPACE_PADDING: usize = 2;
        let mut token_idx_to_start_display_char_idx = Vec::new();
        let mut token_idx_to_end_display_char_idx = Vec::new();
        // write opening display text
        let mut opening_line = String::new();
        {
            // for skipping padding at beginning
            let mut is_first = true;
            for ll_token in self.ll_line.ll_tokens.iter() {
                if is_first {
                    is_first = false;
                } else {
                    opening_line.extend(std::iter::repeat(' ').take(SPACE_PADDING));
                }

                token_idx_to_start_display_char_idx.push(UnicodeWidthStr::width(&*opening_line));

                match &ll_token.token {
                    LToken::Text(text, _) => {
                        opening_line.push_str(text);
                    }
                    LToken::Value { .. } => {
                        write!(&mut opening_line, "<>")?;
                    }
                }

                token_idx_to_end_display_char_idx.push(UnicodeWidthStr::width(&*opening_line));
            }
        }

        f.write_str(&opening_line)?;

        // ex:
        //     ╰────────────╯ Amount(..)
        //                            ╰─╯ Amount(..)
        for ((starts_at_token_idx, ends_at_token_idx), debug_value) in self.include_attrs.iter() {
            f.write_char('\n')?;

            let start_char_idx = token_idx_to_start_display_char_idx[*starts_at_token_idx];
            for _ in 0..start_char_idx {
                f.write_char(' ')?;
            }

            f.write_char('╰')?;

            let end_char_idx = token_idx_to_end_display_char_idx[*ends_at_token_idx];
            let char_len = end_char_idx - start_char_idx;
            for _ in (start_char_idx + 1)..end_char_idx.saturating_sub(1) {
                f.write_char('─')?;
            }

            if char_len > 1 {
                f.write_char('╯')?;
            }

            f.write_str(debug_value)?;
        }

        Ok(())
    }
}

impl<'a> LLLineDisplay<'a> {
    pub fn new(ll_line: &'a LLLine) -> Self {
        LLLineDisplay {
            ll_line,
            include_attrs: Vec::new(),
        }
    }
    // TODO consider making this method take and return `self`
    pub fn include<T: 'static + std::fmt::Debug>(&mut self) {
        for ll_range in self.ll_line.attrs.ranges.get::<T>() {
            for debug_value in self
                .ll_line
                .attrs
                .values
                .get(ll_range)
                .into_iter()
                .flat_map(|type_bucket| type_bucket.get_debug::<T>())
                .rev()
            {
                self.include_attrs.push((*ll_range, debug_value));
            }
        }
    }
    /// Takes self
    pub fn with<T: 'static + std::fmt::Debug>(mut self) -> Self {
        self.include::<T>();
        self
    }
}
