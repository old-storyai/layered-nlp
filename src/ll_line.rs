mod as_tokens;
mod display;
mod ll_selection;
pub(crate) mod x;

pub(crate) use ll_selection::LLSelection;

use unicode_width::UnicodeWidthStr;

use crate::type_bucket::{self, AnyAttribute};
use crate::type_id_to_many::TypeIdToMany;
pub use display::LLLineDisplay;
use std::fmt::Write;
use std::{collections::HashMap, rc::Rc};
use x::XMatchNext;
pub use x::{Attr, AttrEq};

#[derive(Clone, Debug, PartialEq)]
pub enum TextTag {
    NATN,
    PUNC,
    SYMB,
    SPACE,
    WORD,
}

#[derive(Debug)]
pub enum LToken {
    Text(String, TextTag),
    /// TODO: something more interesting
    Value,
}

#[derive(Debug)]
pub struct LLToken {
    pub(crate) token_idx: usize,
    // token span position (not token index)
    pub(crate) pos_starts_at: usize,
    // token span position (not token index)
    pub(crate) pos_ends_at: usize,
    pub(crate) token: LToken,
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
    pub(crate) fn new(ll_tokens: Vec<LLToken>) -> Self {
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
                    if text.chars().count() == 1 {
                        // insert char automatically if just one char
                        attrs.insert((token_idx, token_idx), text.chars().next().unwrap());
                    }
                    // insert TextTag automatically
                    attrs.insert((token_idx, token_idx), tag.clone());
                }
                LToken::Value => {
                    // nothing to do...
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

        let assignments = recognizer.go(LLSelection {
            ll_line: ll_line.clone(),
            start_idx: 0,
            end_idx: ll_line.ll_tokens().len() - 1,
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
            self.attrs.insert((start_idx, end_idx), value);
        }

        self
    }
    pub(crate) fn add_any_attrs(
        &mut self,
        start_idx: usize,
        end_idx: usize,
        attrs: Vec<AnyAttribute>,
    ) {
        let range = (start_idx, end_idx);

        for attr in attrs {
            self.attrs
                .starts_at
                .get_mut(start_idx)
                .expect("has initial starts_at value in bounds")
                .insert_any_distinct(attr.type_id(), range);
            self.attrs
                .ends_at
                .get_mut(end_idx)
                .expect("has initial ends_at value in bounds")
                .insert_any_distinct(attr.type_id(), range);
            self.attrs.ranges.insert_any_distinct(attr.type_id(), range);
            self.attrs
                .values
                .entry(range)
                .or_default()
                .insert_any_attribute(attr);
        }
    }

    /// Get a reference to the l l line's ll tokens.
    pub fn ll_tokens(&self) -> &[LLToken] {
        &self.ll_tokens
    }
}

impl LLLineAttrs {
    fn insert<T: 'static + std::fmt::Debug>(&mut self, range: LRange, value: T) {
        self.starts_at
            .get_mut(range.0)
            .expect("has initial starts_at value in bounds")
            .insert_distinct::<T>(range);
        self.ends_at
            .get_mut(range.1)
            .expect("has initial ends_at value in bounds")
            .insert_distinct::<T>(range);
        self.ranges.insert_distinct::<T>(range);
        self.values.entry(range).or_default().insert(value);
    }
}

#[track_caller]
fn assert_ll_lines_equals(first: &Rc<LLLine>, second: &Rc<LLLine>) {
    if !Rc::ptr_eq(first, second) {
        panic!("Two different lines used")
    }
}

// TODO rename
#[derive(Debug)]
pub struct LLCursorAssignment<Attr> {
    // private
    start_idx: usize,
    end_idx: usize,
    // provided from resolver
    value: Attr,
}

pub trait Resolver {
    type Attr: std::fmt::Debug + 'static;
    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>>;
}
