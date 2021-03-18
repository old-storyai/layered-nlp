#![allow(dead_code, unused_variables)]
use super::*;

impl LLCursorStart {
    pub fn matches_start<'a, M: ItemMatcher<'a>>(
        &'a self,
        matcher: &'a M,
    ) -> Vec<(LLCursor, M::Out)> {
        (0..self.ll_line.ll_tokens.len())
            .map(|i| matcher.go(self, i as isize))
            .flatten()
            .collect()
    }
}

impl LLCursor {
    // Hmmm
    pub fn forwards_group<'a, M: ItemMatcher<'a>, NextAcc, F: Fn(&M::Out) -> Option<NextAcc>>(
        &'a self,
        matcher: M,
        acc: F,
    ) -> CursorGroup<NextAcc> {
        todo!()
    }
}

/// TODO: Consider the [LLOneWayCursor] as a way to forget about whether [ma::TokenIs], [ma::TokenTagged], etc are forwards or backwards.
/// They just go whichever way the user resolver asks of them.
type LLOneWayCursor = LLCursorStart;

pub trait ItemMatcher<'a> {
    type Out;
    fn go(
        &'a self,
        cursor: &'a LLOneWayCursor,
        offset: isize,
    ) -> Option<(super::LLCursor, Self::Out)>;
}

pub mod ma {
    use super::TextTag;

    // pub struct OneOfChar(pub &'static [char]);
    // impl<'a> super::ItemMatcher<'a> for OneOfChar {
    //     type Out = ();
    //     fn go(&self, cursor: &'a super::LLOneWayCursor) -> Option<(super::LLCursor, Self::Out)> {
    //         todo!()
    //     }
    // }

    // pub struct TokenIs(pub &'static str);
    // impl<'a> super::ItemMatcher<'a> for TokenIs {
    //     type Out = ();

    //     /// TODO: Consider the [super::LLOneWayCursor] as a way to forget about whether [TokenIs] is forwards/backwards
    //     fn go(&self, cursor: &'a super::LLOneWayCursor) -> Option<(super::LLCursor, Self::Out)> {
    //         todo!()
    //     }
    // }

    pub struct TokenTagged(pub TextTag);
    impl<'a> super::ItemMatcher<'a> for TokenTagged {
        type Out = &'a str;

        fn go(
            &'a self,
            cursor: &'a super::LLOneWayCursor,
            offset: isize,
        ) -> Option<(super::LLCursor, Self::Out)> {
            cursor
                .ll_line
                .ll_tokens
                .get((cursor.start_at_idx as isize + offset) as usize)
                .and_then(|ll_token| {
                    ll_token.token.has_tag(&self.0).map(|text| {
                        let start_idx = ll_token.token_idx;
                        let end_idx = ll_token.token_idx;
                        (
                            super::LLCursor {
                                start_idx,
                                end_idx,
                                ll_line: cursor.ll_line.clone(),
                            },
                            text,
                        )
                    })
                })
        }
    }

    #[derive(Default)]
    pub struct HasAttr<Attr>(std::marker::PhantomData<Attr>);
    impl<'a, Attr: 'a> super::ItemMatcher<'a> for HasAttr<Attr> {
        type Out = &'a Attr;

        fn go(
            &'a self,
            cursor: &'a super::LLOneWayCursor,
            offset: isize,
        ) -> Option<(crate::ll_line::LLCursor, Self::Out)> {
            todo!()
        }
    }
}

// struct CursorGroup<Acc>(Vec<(Acc, LLCursor)>);
type HMM = ();

enum CurrencySymbol {
    Euros,
    USD,
}

pub struct CursorGroup<Acc>(std::marker::PhantomData<Acc>);
impl<Acc> CursorGroup<Acc> {
    pub fn then_match<'a, M: ItemMatcher<'a>, NextAcc, F: Fn(&Acc, &M::Out) -> Option<NextAcc>>(
        &'a self,
        matcher: M,
        acc: F,
    ) -> CursorGroup<NextAcc> {
        todo!()
    }
    pub fn skip_one<'a, M: ItemMatcher<'a>>(&'a self, matcher: M) -> CursorGroup<Acc> {
        todo!()
    }
    pub fn finish_multiple_matches(self) -> Option<LLCursorAssignment<Acc>> {
        todo!()
    }

    // pub fn filter_map<F, NextAcc>(self, map_fn: F) -> CursorGroup<NextAcc>
    // where
    //     F: Fn(&Acc, &M::Out) -> Option<NextAcc>,
    // {
    //     todo!()
    // }

    fn test_self(self) {
        let a: Option<_> = self
            .then_match(ma::TokenTagged(TextTag::SYMB), |acc, sym| match &**sym {
                "$" => Some(CurrencySymbol::USD),
                "£" => Some(CurrencySymbol::Euros),
                _ => None,
            })
            .finish_multiple_matches();

        //type_check(a);
    }
}

fn type_check((): ()) {}

// inside CurrencyAmountResolver::go
//     // 1 thousand island dressing
//     //
//     start.forwards_group(
//         ma::TokenTagged(&TextTag::NATN),
//         String::from // number string in progress
//     )
//     .repeats(|cursor_group| {
//         cursor_group.skip_one(
//             ma::OneOfChar(self.delimiters.clone()),
//         ).then_match(
//             ma::TokenTagged(&TextTag::NATN),
//             |mut acc, next| {
//                 acc.push_str(next.into());
//                 acc
//             }
//         )
//     })
//     .zero_or_one(|cursor_group| {
//         cursor_group.skip_one(
//             ma::OneOfChar(&[self.decimal.clone()]),
//         ).then_match(
//             ma::TokenTagged(&TextTag::NATN),
//             |mut acc, next| {
//                 acc.push('.');
//                 acc.push_str(next.into());
//                 acc
//             }
//         )
//     })
//     .filter_map(|num_str| Amount(num_str.parse::<Decimal>().ok()))
//     .finish_multiple_matches() // handles overlapping issues
