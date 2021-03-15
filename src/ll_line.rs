use unicode_width::UnicodeWidthStr;

use crate::type_bucket;
use crate::type_id_to_many::TypeIdToMany;
use std::fmt::Write;
use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub enum TextTag {
    NATN,
    PUNC,
    SYMB,
    SPACE,
    OTHER,
}

#[derive(Debug)]
pub enum LToken {
    Text(String, TextTag),
    /// TODO: something more interesting
    Value(String),
}

impl LToken {
    fn has_tag(&self, tag2: &TextTag) -> Option<&str> {
        match self {
            LToken::Text(text, tag1) => {
                if tag1 == tag2 {
                    Some(text)
                } else {
                    None
                }
            }
            LToken::Value(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct LLToken {
    token_idx: usize,
    // token span position (not token index)
    pos_starts_at: usize,
    // token span position (not token index)
    pos_ends_at: usize,
    token: LToken,
}

/// (starts at, ends at) token indexes
type LRange = (usize, usize);

struct LLLineAttrs {
    // "bi-map" / "tri-map"
    ranges: TypeIdToMany<LRange>,
    /// match_forwards uses [LLCursor::end]
    starts_at: Vec<TypeIdToMany<LRange>>,
    /// match_backwards uses [LLCursor::start]
    ends_at: Vec<TypeIdToMany<LRange>>,
    values: HashMap<LRange, type_bucket::TypeBucket>,
}

/// TODO: insert [TextTag]s into the ranges
pub struct LLLine {
    // how much do we actually need of the original Vec if much of the data is put into the bi-map?
    ll_tokens: Vec<LLToken>,
    attrs: LLLineAttrs,
}

impl LLLine {
    pub fn new(ll_tokens: Vec<LLToken>) -> Self {
        let mut starts_at: Vec<TypeIdToMany<LRange>> = Default::default();
        let mut ends_at: Vec<TypeIdToMany<LRange>> = Default::default();
        for _ in ll_tokens.iter() {
            starts_at.push(Default::default());
            ends_at.push(Default::default());
        }
        let mut attrs = LLLineAttrs {
            ranges: Default::default(),
            starts_at,
            ends_at,
            values: Default::default(),
        };

        for (token_idx, ll_token) in ll_tokens.iter().enumerate() {
            match &ll_token.token {
                LToken::Text(text, tag) => {
                    if text.len() == 1 {
                        // insert char automatically if just one char
                        attrs.insert((token_idx, token_idx), text.chars().next().unwrap());
                    }
                    // insert TextTag automatically
                    attrs.insert((token_idx, token_idx), tag.clone());
                }
                LToken::Value(_) => {
                    // todo, insert some initial attr for the value
                }
            }
        }

        LLLine { attrs, ll_tokens }
    }

    pub fn run<R>(mut self, recognizer: &R) -> Self
    where
        R: Resolver,
    {
        let ll_line = Rc::new(self);

        let assignments = recognizer.go(LLCursorStart {
            ll_line: ll_line.clone(),
            start_at_idx: 0,
        });

        self = Rc::try_unwrap(ll_line)
            .map_err(drop)
            .expect("there is no other Rc currently");

        // caches currency symbols
        for LLCursorAssignment {
            start_idx,
            end_idx,
            value,
        } in assignments
        {
            // let ll_line = Rc::get_mut(&mut rc_ll_line).expect("there is no other Rc currently");
            self.attrs.insert((start_idx, end_idx), value);
        }

        self
    }
}

impl LLLineAttrs {
    fn insert<T: 'static + std::fmt::Debug>(&mut self, range: LRange, value: T) {
        self.starts_at
            .get_mut(range.0)
            .expect("has initial starts_at value in bounds")
            .insert::<T>(range);
        self.ends_at
            .get_mut(range.1)
            .expect("has initial ends_at value in bounds")
            .insert::<T>(range);
        self.ranges.insert::<T>(range);
        self.values.entry(range).or_default().insert(value);
    }
}

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
                        opening_line.push_str(&text);
                    }
                    LToken::Value(to_print) => {
                        write!(&mut opening_line, "<{}>", to_print)?;
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

            f.write_str(&debug_value)?;
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
    pub fn include<T: 'static + std::fmt::Debug + Clone>(&mut self) {
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
                self.include_attrs.push((ll_range.clone(), debug_value));
            }
        }
    }
}

#[derive(Clone)]
pub struct LLCursor {
    // private
    start_idx: usize,
    end_idx: usize,
    ll_line: Rc<LLLine>,
}

pub struct LLCursorStart {
    ll_line: Rc<LLLine>,
    /// Where to begin in the line (inclusive, default is 0)
    start_at_idx: usize,
}

#[derive(Debug)]
pub struct LLCursorAssignment<Attr> {
    // private
    start_idx: usize,
    end_idx: usize,
    // provided from resolver
    value: Attr,
}

impl LLCursorStart {
    // really relaxed, uncomfortably so.
    pub fn find_start_eq<T: PartialEq>(&self, _attr: &T) -> Vec<LLCursor> {
        unimplemented!()
    }

    // really relaxed, uncomfortably so.
    pub fn find_start_tag(&self, find_tag: &TextTag) -> Vec<(LLCursor, &str)> {
        self.ll_line.ll_tokens[self.start_at_idx..]
            .iter()
            .filter_map(|ll_token| {
                if let LToken::Text(text, tag) = &ll_token.token {
                    if tag == find_tag {
                        Some((
                            LLCursor {
                                start_idx: ll_token.token_idx,
                                end_idx: ll_token.token_idx,
                                ll_line: self.ll_line.clone(),
                            },
                            text.as_str(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_next_start_tag(&self, find_tag: &TextTag) -> Option<(LLCursor, &str)> {
        // Not optimal, but okay for now
        self.find_start_tag(find_tag).into_iter().next()
    }

    pub fn find_start<Attr: 'static + std::fmt::Debug>(&self) -> Vec<(LLCursor, &Attr)> {
        self.ll_line
            .attrs
            .values
            .iter()
            .filter_map(|(&(start, end), value)| {
                let attrs = value.get::<Attr>();
                if !attrs.is_empty() {
                    Some(attrs.iter().map(move |attr| {
                        (
                            LLCursor {
                                start_idx: start,
                                end_idx: end,
                                ll_line: self.ll_line.clone(),
                            },
                            attr,
                        )
                    }))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
}

impl LLCursor {
    pub fn start_after(&self) -> LLCursorStart {
        LLCursorStart {
            ll_line: self.ll_line.clone(),
            start_at_idx: self.end_idx + 1,
        }
    }

    pub fn match_forwards<Attr: 'static>(&self) -> Vec<(LLCursor, &Attr)> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        //        [...] - Trying to match Attr
        if self.end_idx + 1 == self.ll_line.ll_tokens.len() {
            return Vec::new();
        }

        self.ll_line
            .attrs
            .starts_at
            .get(self.end_idx + 1)
            .expect("Huh... match_forwards was at the end")
            .get::<Attr>()
            .iter()
            .flat_map(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<Attr>()
                    .iter()
                    .map(move |val| (val, range.1))
            })
            .map(|(val, end_idx)| {
                (
                    LLCursor {
                        start_idx: self.start_idx,
                        end_idx,
                        ll_line: self.ll_line.clone(),
                    },
                    val,
                )
            })
            .collect()
    }
    pub fn match_forwards_char(&self, c: &[char]) -> Option<LLCursor> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        //        [...] - Trying to match Attr
        if self.end_idx + 1 == self.ll_line.ll_tokens.len() {
            return None;
        }

        self.ll_line
            .attrs
            .starts_at
            .get(self.end_idx + 1)
            .expect("Huh... match_forwards_char was at the end")
            .get::<char>()
            .iter()
            .flat_map(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<char>() // probably optimizable since there is only going to ever be one potential char per token
                    .iter()
                    .map(move |val| (val, range.1))
            })
            .filter_map(|(val, end_idx)| {
                if c.contains(&val) {
                    Some(LLCursor {
                        start_idx: self.start_idx,
                        end_idx,
                        ll_line: self.ll_line.clone(),
                    })
                } else {
                    None
                }
            })
            .next()
    }
    pub fn match_forwards_tag(&self, tag: &TextTag) -> Option<(LLCursor, &str)> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        //        [...] - Trying to match Attr
        if self.end_idx + 1 == self.ll_line.ll_tokens.len() {
            return None;
        }

        self.ll_line
            .ll_tokens
            .get(self.end_idx + 1)
            .expect("Huh... match_forwards_tag was at the end")
            .token
            .has_tag(&tag)
            .map(|val| {
                (
                    LLCursor {
                        start_idx: self.start_idx,
                        end_idx: self.end_idx + 1,
                        ll_line: self.ll_line.clone(),
                    },
                    val,
                )
            })
    }
    pub fn match_backwards<Attr: 'static>(&self) -> Vec<(LLCursor, &Attr)> {
        //        [ ... ] - Current Cursor
        // [ ... ] - Trying to match Attr
        //   [...] - Trying to match Attr
        if self.start_idx == 0 {
            return Vec::new();
        }

        let end_idx = self.start_idx - 1;
        self.ll_line
            .attrs
            .ends_at
            .get(end_idx)
            .expect("Huh... match_backwards was at the start")
            .get::<Attr>()
            .iter()
            .flat_map(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<Attr>()
                    .iter()
                    .map(move |val| (val, range.0))
            })
            .map(|(val, start_idx)| {
                (
                    LLCursor {
                        start_idx,
                        end_idx: self.end_idx,
                        ll_line: self.ll_line.clone(),
                    },
                    val,
                )
            })
            .collect()
    }
    pub fn match_backwards_char(&self, c: &[char]) -> Option<LLCursor> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        //        [...] - Trying to match Attr
        if self.start_idx == 0 {
            return None;
        }

        self.ll_line
            .attrs
            .ends_at
            .get(self.start_idx - 1)
            .expect("Huh... match_backwards was at the start")
            .get::<char>()
            .iter()
            .flat_map(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<char>() // probably optimizable since there is only going to ever be one potential char per token
                    .iter()
                    .rev()
                    .map(move |val| (val, range.0))
            })
            .filter_map(|(val, start_idx)| {
                if c.contains(&val) {
                    Some(LLCursor {
                        start_idx,
                        end_idx: self.end_idx,
                        ll_line: self.ll_line.clone(),
                    })
                } else {
                    None
                }
            })
            .next()
    }
    // fn match_forwards_skip_spaces<Attr>(&self) -> Option<(LLCursor, &Attr)> {
    //     unimplemented!()
    // }
    // fn match_backwards_skip_spaces<Attr>(&self) -> Option<(LLCursor, &Attr)> {
    //     unimplemented!()
    // }
    pub fn finish<Attr>(&self, value: Attr) -> LLCursorAssignment<Attr> {
        LLCursorAssignment {
            end_idx: self.end_idx,
            start_idx: self.start_idx,
            value,
        }
    }
}

pub trait Resolver {
    type Attr: std::fmt::Debug + 'static;
    fn go(&self, cursor: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>>;
}

#[cfg(test)]
pub fn ll(
    token_idx: usize,
    pos_starts_at: usize,
    pos_ends_at: usize,
    tagged: TextTag,
    text: &str,
) -> LLToken {
    LLToken {
        pos_starts_at,
        pos_ends_at,
        token_idx,
        token: LToken::Text(text.into(), tagged),
    }
}
