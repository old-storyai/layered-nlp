#![allow(dead_code)]

use crate::type_bucket;
use crate::type_id_to_many::TypeIdToMany;
use insta;
use std::fmt::Write;
use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
enum TextTag {
    NATN,
    PUNC,
    SYMB,
    SPACE,
    OTHER,
}

#[derive(Debug)]
enum LToken {
    Text(String, TextTag),
    /// TODO: something more interesting
    Value(String),
}

#[derive(Debug)]
struct LLToken {
    // token span position (not token index)
    pos_starts_at: usize,
    // token span position (not token index)
    pos_ends_at: usize,
    token: LToken,
}

fn ll(pos_starts_at: usize, pos_ends_at: usize, tagged: TextTag, text: &str) -> LLToken {
    LLToken {
        pos_starts_at,
        pos_ends_at,
        token: LToken::Text(text.into(), tagged),
    }
}

fn ll_usd_1000_25() -> Vec<LLToken> {
    vec![
        ll(0, 1, TextTag::SYMB, "$"),
        ll(1, 5, TextTag::NATN, "1000"),
        ll(5, 6, TextTag::PUNC, "."),
        ll(6, 8, TextTag::NATN, "25"),
    ]
}

#[derive(Clone)]
struct Amount(f64);
#[derive(Clone)]
enum CurrencySymbol {
    USDDollars,
    USDCents,
}
struct CurrencyAmount(CurrencySymbol, Amount);

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
struct LLLine {
    // how much do we actually need of the original Vec if much of the data is put into the bi-map?
    ll_tokens: Vec<LLToken>,
    attrs: LLLineAttrs,
}

impl LLLine {
    fn new(ll_tokens: Vec<LLToken>) -> Self {
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
                LToken::Text(_text, tag) => {
                    attrs.insert((token_idx, token_idx), tag.clone());
                }
                LToken::Value(_) => {
                    // todo, insert some initial attr for the value
                }
            }
        }

        LLLine { attrs, ll_tokens }
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

struct LLLineDisplay {
    ll_line: Rc<LLLine>,
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
impl std::fmt::Display for LLLineDisplay {
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

                token_idx_to_start_display_char_idx.push(opening_line.len());

                match &ll_token.token {
                    LToken::Text(text, _) => {
                        opening_line.push_str(&text);
                    }
                    LToken::Value(to_print) => {
                        write!(&mut opening_line, "<{}>", to_print)?;
                    }
                }

                token_idx_to_end_display_char_idx.push(opening_line.len());
            }
        }

        f.write_str(&opening_line)?;

        // ex:
        //     ╰────────────╯ Amount(..)
        //                            ╰─╯ Amount(..)
        for ((starts_at_token_idx, ends_at_token_idx), debug_value) in
            self.include_attrs.iter().rev()
        {
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

impl LLLineDisplay {
    pub fn new(ll_line: Rc<LLLine>) -> Self {
        LLLineDisplay {
            ll_line,
            include_attrs: Vec::new(),
        }
    }
    pub fn include<T: 'static + std::fmt::Debug + Clone>(&mut self) {
        for ll_range in self.ll_line.attrs.ranges.get::<T>() {
            if let Some(debug_value) = self
                .ll_line
                .attrs
                .values
                .get(ll_range)
                .and_then(|type_bucket| type_bucket.get_debug::<T>())
            {
                self.include_attrs.push((ll_range.clone(), debug_value));
            }
        }
    }
}

struct LLCursor {
    // private
    start: usize,
    end: usize,
    ll_line: Rc<LLLine>,
}

struct LLCursorStart {
    ll_line: Rc<LLLine>,
}

struct LLCursorAssignment<Attr> {
    // private
    start: usize,
    end: usize,
    // provided from resolver
    value: Attr,
}

impl LLCursorStart {
    // really relaxed, uncomfortably so.
    fn find_start_eq<T: PartialEq>(&self, _attr: &T) -> Vec<LLCursor> {
        unimplemented!()
    }

    // really relaxed, uncomfortably so.
    fn find_start_tag(&self, _tag: &TextTag) -> Vec<(LLCursor, &str)> {
        unimplemented!()
    }

    fn find_start<Attr>(&self) -> Vec<(LLCursor, &Attr)> {
        unimplemented!()
    }
}

impl LLCursor {
    fn match_forwards<Attr>(&self) -> Option<(LLCursor, &Attr)> {
        unimplemented!()
    }
    fn match_backwards<Attr>(&self) -> Option<(LLCursor, &Attr)> {
        unimplemented!()
    }
    // fn match_forwards_skip_spaces<Attr>(&self) -> Option<(LLCursor, &Attr)> {
    //     unimplemented!()
    // }
    // fn match_backwards_skip_spaces<Attr>(&self) -> Option<(LLCursor, &Attr)> {
    //     unimplemented!()
    // }
    fn finish<Attr>(&self, value: Attr) -> LLCursorAssignment<Attr> {
        LLCursorAssignment {
            end: self.end,
            start: self.start,
            value,
        }
    }
}

trait Resolver {
    type Attr;
    fn go(cursor: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>>;
}

struct CurrencySymbolResolver;
impl Resolver for CurrencySymbolResolver {
    type Attr = CurrencySymbol;

    // fn resolve(ll_tokens: Vec<LLToken>) -> Option<CurrencySymbol> {}

    fn go(cursor: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
        cursor
            .find_start_tag(&TextTag::SYMB)
            .into_iter()
            .filter_map(|(cur, sym_str)| {
                if let "$" = sym_str {
                    Some(cur.finish(CurrencySymbol::USDDollars))
                } else {
                    None
                }
            })
            .collect()
    }
}

struct CurrencyAmountResolver;
impl Resolver for CurrencyAmountResolver {
    type Attr = CurrencyAmount;

    fn go(cursor: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
        cursor
            .find_start::<CurrencySymbol>()
            .into_iter()
            .filter_map(|(cur, curr_sym)| {
                // curr_sym
                cur.match_forwards::<Amount>()
                    .or_else(|| cur.match_backwards::<Amount>())
                    .map(|(cur_with_amt, amt)| {
                        cur_with_amt.finish(CurrencyAmount(curr_sym.clone(), amt.clone()))
                    })
            })
            .collect()
    }
}

#[test]
fn it_works() {
    let input = ll_usd_1000_25();
    let ll_line = LLLine::new(input);
    let rc_ll_line = Rc::new(ll_line);
    let mut ll_line_display = LLLineDisplay::new(rc_ll_line);
    ll_line_display.include::<TextTag>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    $  1000  .  25
                ╰╯[NATN]
             ╰[PUNC]
       ╰──╯[NATN]
    ╰[SYMB]
    "###);
    // match_forwards::<CurrencySymbol>(0) -> Some((CurrencySymbol::USDDollars, 1))
    // match_forwards::<Amount>(1) -> Some((Amount(1000f64), 8))
    // match_backwards::<Amount>(8) -> Some((Amount(1000f64), 1))
}
