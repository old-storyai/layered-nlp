#![allow(dead_code)]

use crate::type_bucket;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
};

#[derive(Clone)]
enum TextTag {
    NATN,
    PUNC,
    SYMB,
    SPACE,
    OTHER,
}

enum LToken {
    Text(String, TextTag),
    Value(String),
}

struct LLToken {
    starts_at: usize,
    ends_at: usize,
    token: LToken,
}

fn ll(starts_at: usize, ends_at: usize, tagged: TextTag, text: &str) -> LLToken {
    LLToken {
        starts_at,
        ends_at,
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
    // USDCents,
}
struct CurrencyAmount(CurrencySymbol, Amount);

type LRange = (usize, usize);

/// TODO: insert [TextTag]s into the ranges
struct LLLine {
    ll_tokens: Vec<LLToken>,
    // some type map range lookup thing
    // attrs: BTreeMap<usize, Box<dyn Any>>,
    // "bi-map"
    ranges: HashMap<TypeId, Vec<LRange>>,
    values: HashMap<LRange, type_bucket::TypeBucket>,
    // annotations: BTreeMap<
    //     // start token index (inclusive)
    //     usize,
    //     BTreeMap< // bi-map?
    //         // end bound (exclusive)
    //         usize,
    //         // attributes
    //         type_map::TypeBucket,
    //     >,
    // >
}

type TODO = ();

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
    fn find_start_tag(&self, tag: &TextTag) -> Vec<(LLCursor, &str)> {
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

    // match_forwards::<CurrencySymbol>(0) -> Some((CurrencySymbol::USDDollars, 1))
    // match_forwards::<Amount>(1) -> Some((Amount(1000f64), 8))
    // match_backwards::<Amount>(8) -> Some((Amount(1000f64), 1))
}
