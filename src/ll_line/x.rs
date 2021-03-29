mod any_of;
mod functions;
mod seq;

pub use any_of::{AnyOf, AnyOf2, AnyOf3};
pub use functions::{any_of, attr, attr_eq, seq, token_has_any, token_text};
pub use seq::{Seq, Seq2, Seq3};

use super::{LLLine, LLToken, LToken};

/// Examples: Attr, AttrEq
pub trait XMatch<'l> {
    /// Usually must be [Copy] so it's compatible with any multi-matchers.
    /// The Out must be copied in the event of "cartesian" product scenarios where multi-matchers
    /// return multiple combinations of their inner matchers' Out.
    type Out: Copy;
    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>;
}

#[derive(PartialEq, Clone, Copy)]
pub struct ToIdx(pub(crate) usize);

pub struct TokenText(());

impl<'l> XMatch<'l> for TokenText {
    type Out = &'l str;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        direction.text_token(ll_line).into_iter().collect()
    }
}

pub struct AttrEq<'a, Attr> {
    attr: &'a Attr,
}

impl<'l, Attr: PartialEq + 'static> XMatch<'l> for AttrEq<'_, Attr> {
    type Out = ();

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        direction.attr_eq(self.attr, ll_line)
    }
}
pub struct TokenHasAny<'a, Attr: PartialEq> {
    one_of: &'a [Attr],
}

impl<'l, Attr: PartialEq + 'static> XMatch<'l> for TokenHasAny<'_, Attr> {
    type Out = &'l Attr;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        direction.token_attr_one_of(self.one_of, ll_line)
    }
}

pub struct Attr<Attr>(std::marker::PhantomData<Attr>);

impl<'l, A: 'static> XMatch<'l> for Attr<A> {
    type Out = &'l A;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        direction.attr::<A>(ll_line)
    }
}

pub trait XDirection<'l>
where
    Self: Sized,
{
    // hmm... succeeding, next_after_idx? Consolidate boundary checking?
    // #[derive(Debug)]
    // pub struct OutOfBoundsError;
    // /// Includes bounds checking, returns None if out of bounds
    // fn from_idx(ll_line: &'l LLLine, idx: usize) -> Result<Self, OutOfBoundsError> {
    //     todo!()
    // }
    fn attr<T: 'static>(&self, ll_line: &'l LLLine) -> Vec<(&'l T, ToIdx)>;
    fn attr_eq<T: 'static + PartialEq>(&self, equals: &T, ll_line: &'l LLLine) -> Vec<((), ToIdx)>;
    fn token_attr_one_of<T: 'static + PartialEq>(
        &self,
        set: &[T],
        ll_line: &'l LLLine,
    ) -> Vec<(&'l T, ToIdx)>;
    /// If the next token is Text, return the inner string slice
    fn text_token(&self, ll_line: &'l LLLine) -> Option<(&'l str, ToIdx)>;
    fn after(&self, idx: usize, ll_line: &'l LLLine) -> Option<Self>
    where
        Self: Sized;
}

pub(crate) struct XForwards {
    pub(super) from_idx: usize,
}

// TODO:
//        ╰─╯Amount(1000.25)
//  ╰───╯Person(A)
//                 ╰───╯Person(B)
//           ╰───╯VerbPhrase
// selection
//  .find_by(attr::<Clause>())
//  .iter ... .map
//      .contains_in_any_order(&(multiple::<Person, 2>(), attr::<Amount>(), attr::<VerbPhrase>()))
//        -> Vec of 2 -> Full original selection
//             - ([&Person(A), &Person(B)], &Amount, &VerbPhrase)
// pub(crate) struct XContains {
//     pub(super) start_idx: usize,
//     pub(super) end_idx: usize,
// }

impl<'l> XDirection<'l> for XForwards {
    fn attr_eq<T: 'static + PartialEq>(&self, equals: &T, ll_line: &'l LLLine) -> Vec<((), ToIdx)> {
        ll_line
            .attrs
            .starts_at
            .get(self.from_idx)
            .expect("Huh... match_forwards was at the end")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .filter_map(move |val| {
                        if val == equals {
                            Some(((), ToIdx(range.1)))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }

    fn attr<T: 'static>(&self, ll_line: &'l LLLine) -> Vec<(&'l T, ToIdx)> {
        ll_line
            .attrs
            .starts_at
            .get(self.from_idx)
            .expect("Huh... match_forwards was at the end")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .map(move |val| (val, ToIdx(range.1)))
            })
            .collect()
    }

    fn token_attr_one_of<T: 'static + PartialEq>(
        &self,
        set: &[T],
        ll_line: &'l LLLine,
    ) -> Vec<(&'l T, ToIdx)> {
        // [ ... ] - Current Selection
        //        [ ... ] - Trying to match Attr
        let next_token_idx = self.from_idx;
        if next_token_idx == ll_line.ll_tokens.len() {
            return Vec::new();
        }

        ll_line
            .attrs
            .values
            .get(&(next_token_idx, next_token_idx))
            .expect("Huh... match_forwards was at the end")
            .get::<T>()
            .iter()
            .filter_map(|value| {
                if set.contains(value) {
                    Some((value, ToIdx(next_token_idx)))
                } else {
                    None
                }
            })
            .collect()
    }

    fn text_token(&self, ll_line: &'l LLLine) -> Option<(&'l str, ToIdx)> {
        // [ ... ] - Current Selection
        //        [ ... ] - Trying to match Attr
        match ll_line
            .ll_tokens
            .get(self.from_idx)
            .expect("Huh... XForwards::next_text_token was at the end")
        {
            LLToken {
                token: LToken::Text(ref s, _),
                ..
            } => Some((s, ToIdx(self.from_idx))),
            _ => None,
        }
    }

    fn after(&self, idx: usize, ll_line: &'l LLLine) -> Option<Self> {
        let next_idx = idx + 1;
        if next_idx < ll_line.ll_tokens.len() {
            Some(XForwards { from_idx: next_idx })
        } else {
            None
        }
    }
}

pub(crate) struct XBackwards {
    pub(super) from_idx: usize,
}

impl<'l> XDirection<'l> for XBackwards {
    fn attr_eq<T: 'static + PartialEq>(&self, equals: &T, ll_line: &'l LLLine) -> Vec<((), ToIdx)> {
        //        [ ... ] - Current Selection
        // [ ... ] - Trying to match Attr
        //   [...] - Trying to match Attr
        ll_line
            .attrs
            .ends_at
            .get(self.from_idx)
            .expect("Huh... match_backwards was at the end")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .filter_map(move |val| {
                        if val == equals {
                            Some(((), ToIdx(range.0)))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }

    fn attr<T: 'static>(&self, ll_line: &'l LLLine) -> Vec<(&'l T, ToIdx)> {
        //        [ ... ] - Current Selection
        // [ ... ] - Trying to match Attr
        //   [...] - Trying to match Attr
        ll_line
            .attrs
            .ends_at
            .get(self.from_idx)
            .expect("Huh... Backwards::next_attr was at the start")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .map(move |val| (val, ToIdx(range.0)))
            })
            .collect()
    }

    fn token_attr_one_of<T: 'static + PartialEq>(
        &self,
        set: &[T],
        ll_line: &'l LLLine,
    ) -> Vec<(&'l T, ToIdx)> {
        //        [ ... ] - Current Selection
        // [ ... ] - Trying to match Attr
        //   [...] - Trying to match Attr
        if self.from_idx == 0 {
            return Vec::new();
        }

        let next_token_idx = self.from_idx - 1;
        ll_line
            .attrs
            .ends_at
            .get(next_token_idx)
            .expect("Huh... Backwards::next_attr was at the start")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .filter_map(move |val| {
                        if set.contains(&val) {
                            Some((val, ToIdx(range.0)))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }

    fn text_token(&self, ll_line: &'l LLLine) -> Option<(&'l str, ToIdx)> {
        ll_line
            .ll_tokens()
            .get(self.from_idx)
            .and_then(|token| match &token.token {
                LToken::Text(text, _) => Some((text.as_str(), ToIdx(self.from_idx))),
                LToken::Value => None,
            })
    }

    fn after(&self, idx: usize, _: &'l LLLine) -> Option<Self> {
        dbg!(idx);
        if idx > 0 {
            Some(XBackwards {
                from_idx: self.from_idx - 1,
            })
        } else {
            None
        }
    }
}
