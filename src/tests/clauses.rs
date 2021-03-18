use crate::create_tokens::InputToken;
use crate::type_bucket::AnyAttribute;

use super::*;

#[derive(Clone, Debug)]
enum Clause {
    LeadingEffect,
    TrailingEffect,
    Condition,
    Independent,
}

struct ClauseResolver {
    // hmmm... configuration?
}

// impl Resolver for ClauseResolver {
//     type Attr = Clause;

//     fn go(&self, mut start: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
//         todo!()
//         // start
//         //     .find_next_start_eq(&ClauseKeyword::And)
//         //     .into_iter()
//         //     .flat_map(|(sentence_start, _)| {
//         //         std::iter::once(
//         //             {
//         //                 match (sentence_start.match_forwards_until_before_eq_or_until_end_of_line(
//         //                     &ClauseKeyword::ConditionStart,
//         //                 ), sentence_start.match_forwards_until_before_eq_or_until_end_of_line(
//         //                     &ClauseKeyword::And,
//         //                 )) {
//         //                     (MatchResult::Some(cur1, _), MatchResult::Some(cur2, _)) => {
//         //                         if cur1.start_before(&cur2) {
//         //                             Some(cur1.finish_with_attr(Clause::LeadingEffect))
//         //                         } else {
//         //                             Some(cur2.finish_with_attr(Clause::LeadingEffect))
//         //                         }
//         //                     }
//         //                     (MatchResult::Some(cur, _), MatchResult::None) => {
//         //                         Some(cur1.finish(Clause::LeadingEffect))
//         //                     }
//         //                     (MatchResult::Some(_, _), MatchResult::EndOfLine(_, _)) => {}
//         //                     (MatchResult::None, MatchResult::Some(_, _)) => {}
//         //                     (MatchResult::None, MatchResult::None) => {}
//         //                     (MatchResult::None, MatchResult::EndOfLine(_, _)) => {}
//         //                     (MatchResult::EndOfLine(_, _), MatchResult::Some(_, _)) => {}
//         //                     (MatchResult::EndOfLine(_, _), MatchResult::None) => {}
//         //                     (MatchResult::EndOfLine(_, _), MatchResult::EndOfLine(_, _)) => {}
//         //                 }

//         //                 match sentence_start.match_forwards_until_before_eq_or_until_end_of_line(
//         //                     &ClauseKeyword::ConditionStart,
//         //                 ) {
//         //                     MatchResult::Some(cur, _) => {
//         //                         match sentence_start.match_forwards_until_before_eq_or_until_end_of_line(
//         //                             &ClauseKeyword::ConditionStart,
//         //                         ) {
//         //                             MatchResult::Some(cur1, _) => {
//         //                                 if cur.start_before(&cur1) {
//         //                                     Some(cur.finish_with_attr(Clause::LeadingEffect))
//         //                                 } else {
//         //                                     Some(cur1.finish_with_attr(Clause::LeadingEffect))
//         //                                 }
//         //                             },
//         //                             MatchResult::None => None,
//         //                             MatchResult::EndOfLine(cur1, _) => Some(cur1.finish_with_attr(Clause::Independent)),
//         //                         }
//         //                     },
//         //                     MatchResult::None => None,
//         //                     MatchResult::EndOfLine(cur, _) => {
//         //                         match sentence_start.match_forwards_until_before_eq_or_until_end_of_line(
//         //                             &ClauseKeyword::ConditionStart,
//         //                         ) {
//         //                             MatchResult::Some(cur1, _) => {
//         //                                 if cur1.start_before(&cur) {
//         //                                     Some(cur.finish_with_attr(Clause::LeadingEffect))
//         //                                 } else {
//         //                                     Some(cur.finish_with_attr(Clause::Independent))
//         //                                 }
//         //                             },
//         //                             MatchResult::None => None,
//         //                             MatchResult::EndOfLine(cur1, _) => Some(cur1.finish_with_attr(Clause::Independent)),
//         //                         }
//         //                     }
//         //                 }
//         //             }
//         //         )
//         //         .flatten()
//         //         .chain(
//         //             start
//         //                 .find_start_eq(&ClauseKeyword::ConditionStart)
//         //                 .into_iter()
//         //                 .flat_map(|start_at_condition| {
//         //                     let (cur_matched, _idk) = match start_at_condition
//         //                         .match_forwards_until_before_eq_or_until_end_of_line(
//         //                             &ClauseKeyword::Then,
//         //                         ) {
//         //                         MatchResult::Some(cur, value)
//         //                         | MatchResult::EndOfLine(cur, value) => (cur, value),
//         //                         MatchResult::None => {
//         //                             panic!()
//         //                         }
//         //                     };

//         //                     std::iter::once(cur_matched.finish_with_attr(Clause::Condition)).chain(
//         //                         cur_matched
//         //                             .start_after()
//         //                             .find_next_start_tag(&TextTag::WORD)
//         //                             .into_iter()
//         //                             .map(|(cur_after_cond, _)| {
//         //                                 let trailing_cursor = match cur_after_cond
//         //                                     .match_forwards_until_before_eq_or_until_end_of_line(
//         //                                         &ClauseKeyword::And,
//         //                                     ) {
//         //                                     MatchResult::Some(cur, _)
//         //                                     | MatchResult::EndOfLine(cur, _) => cur,
//         //                                     MatchResult::None => {
//         //                                         panic!()
//         //                                     }
//         //                                 };
//         //                                 // let trailing_cursor = cur_after_cond
//         //                                 //     .match_forwards_until_before_eq_or_until_end_of_line(
//         //                                 //         &ClauseKeyword::And,
//         //                                 //     )
//         //                                 //     .0;

//         //                                 std::iter::once(
//         //                                     trailing_cursor.finish_with_attr(Clause::TrailingEffect),
//         //                                 )
//         //                                 .chain({
//         //                                     trailing_cursor
//         //                                         .start_after()
//         //                                         .find_start_eq(&ClauseKeyword::And)
//         //                                         .into_iter()
//         //                                         .map(|cur| {
//         //                                             cur.match_forwards_until_end_of_line()
//         //                                                 .finish_with_attr(Clause::TrailingEffect)
//         //                                         })
//         //                                 })
//         //                             })
//         //                             .flatten()
//         //                             .collect::<Vec<_>>()
//         //                             .into_iter(),
//         //                     )
//         //                 }),
//         //         )
//         //     })
//         //     .collect()
//     }
// }

#[derive(Debug, Clone, PartialEq)]
enum ClauseKeyword {
    /// "and"
    And,
    /// "if", "when"
    ConditionStart,
    /// "then"
    Then,
}

// struct ClauseKeywordResolver;

// impl Resolver for ClauseKeywordResolver {
//     type Attr = ClauseKeyword;

//     fn go(&self, cursor: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
//         cursor
//             .find_start_eq(&POSTag::Cond)
//             .into_iter()
//             .map(|cur| cur.finish_with_attr(ClauseKeyword::ConditionStart))
//             .chain(
//                 cursor
//                     .find_start_eq(&POSTag::CondThen)
//                     .into_iter()
//                     .map(|cur| cur.finish_with_attr(ClauseKeyword::Then)),
//             )
//             .chain(
//                 cursor
//                     .find_start_eq(&POSTag::And)
//                     .into_iter()
//                     .map(|cur| cur.finish_with_attr(ClauseKeyword::And)),
//             )
//             .collect()
//     }
// }

#[derive(Debug, Clone, PartialEq)]
enum POSTag {
    QuestionWord,
    Cond,
    CondThen,
    And,
    Verb,
    Pronoun,
    Noun,
    Adjective,
    Other,
}

// fn tc(tokens: &[(&str, &[POSTag])]) -> String {
//     let ll_line = create_tokens(
//         tokens
//             .iter()
//             .map(|(text, pos_tags)| {
//                 InputToken::text(
//                     text.to_string(),
//                     pos_tags.iter().cloned().map(AnyAttribute::new).collect(),
//                 )
//             })
//             .collect(),
//         |text| text.encode_utf16().count(),
//     )
//     .run(&ClauseKeywordResolver {})
//     .run(&ClauseResolver {});

//     let mut ll_line_display = LLLineDisplay::new(&ll_line);
//     // ll_line_display.include::<TextTag>();
//     ll_line_display.include::<ClauseKeyword>();
//     ll_line_display.include::<Clause>();
//     ll_line_display.include::<POSTag>();

//     format!("{}", ll_line_display)
// }

// macro_rules! tc {
//     ($($text: literal, $($tokens: ident,)*)+) => {
//         tc(&[
//             $(
//                 ($text, &[$($tokens,)*]),
//             )+
//         ])
//     }
// }

// #[test]
// fn test_clauses() {
//     use POSTag::*;

//     insta::assert_display_snapshot!(
//         tc!(
//             "When", Cond, QuestionWord,
//             " ",
//             "it", Pronoun,
//             " ",
//             "rains", Verb,
//             " ",
//             "then", CondThen,
//             " ",
//             "it", Pronoun,
//             " ",
//             "pours", Verb,
//             ".",
//         ), @r###"
//     When     it     rains     then     it     pours  .
//     ╰──╯ConditionStart
//                               ╰──╯Then
//     ╰──────────────────────╯Condition
//                               ╰──────────────────────╯TrailingEffect
//     ╰──╯QuestionWord
//     ╰──╯Cond
//              ╰╯Pronoun
//                     ╰───╯Verb
//                               ╰──╯CondThen
//                                        ╰╯Pronoun
//                                               ╰───╯Verb
//     "###);

//     insta::assert_display_snapshot!(
//         tc!(
//             "If", Cond,
//             " ",
//             "it", Pronoun,
//             " ",
//             "is", Verb,
//             " ",
//             "raining", Verb,
//             ",",
//             " ",
//             "then", CondThen,
//             " ",
//             "open", Verb,
//             " ",
//             "the", Other,
//             " ",
//             "umbrella", Noun,
//             ".",
//         ), @r###"
//     If     it     is     raining  ,     then     open     the     umbrella  .
//     ╰╯ConditionStart
//                                         ╰──╯Then
//     ╰────────────────────────────────╯Condition
//                                         ╰───────────────────────────────────╯TrailingEffect
//     ╰╯Cond
//            ╰╯Pronoun
//                   ╰╯Verb
//                          ╰─────╯Verb
//                                         ╰──╯CondThen
//                                                  ╰──╯Verb
//                                                           ╰─╯Other
//                                                                   ╰──────╯Noun
//     "###);
// }

// #[test]
// fn tired() {
//     use POSTag::*;

//     insta::assert_display_snapshot!(
//         tc!(
//             "Si", Cond,
//             " ",
//             "tu", Pronoun,
//             " ",
//             "es", Verb,
//             " ",
//             "fatigué", Adjective,
//             ",", CondThen,
//             " ",
//             "va", Verb,
//             " ",
//             "te", Other,
//             " ",
//             "coucher", Verb,
//             ".",
//         ), @r###"
//     Si     tu     es     fatigué  ,     va     te     coucher  .
//     ╰╯ConditionStart
//                                   ╰Then
//     ╰──────────────────────────╯Condition
//                                         ╰──────────────────────╯TrailingEffect
//     ╰╯Cond
//            ╰╯Pronoun
//                   ╰╯Verb
//                          ╰─────╯Adjective
//                                   ╰CondThen
//                                         ╰╯Verb
//                                                ╰╯Other
//                                                       ╰─────╯Verb
//     "###);
// }

// #[test]
// fn tired_rev() {
//     use POSTag::*;

//     insta::assert_display_snapshot!(
//         tc!(
//             "Va", Verb,
//             " ",
//             "te", Other,
//             " ",
//             "coucher", Verb,
//             ",",
//             " ",
//             "si", Cond,
//             " ",
//             "tu", Pronoun,
//             " ",
//             "es", Verb,
//             " ",
//             "fatigué", Adjective,
//             ".",
//         ), @r###"
//     Va     te     coucher  ,     si     tu     es     fatigué  .
//                                  ╰╯ConditionStart
//     ╰─────────────────────────╯LeadingEffect
//                                  ╰─────────────────────────────╯Condition
//     ╰╯Verb
//            ╰╯Other
//                   ╰─────╯Verb
//                                  ╰╯Cond
//                                         ╰╯Pronoun
//                                                ╰╯Verb
//                                                       ╰─────╯Adjective
//     "###);
// }

// #[test]
// fn rain() {
//     use POSTag::*;

//     insta::assert_display_snapshot!(
//         tc!(
//             "If", Cond,
//             " ",
//             "it", Pronoun,
//             " ",
//             "is", Verb,
//             " ",
//             "raining", Verb,
//             " ",
//             "then", CondThen,
//             " ",
//             "open", Verb,
//             " ",
//             "the",
//             " ",
//             "umbrella", Noun,
//             " ",
//             "and", And,
//             " ",
//             "close", Verb,
//             " ",
//             "the",
//             " ",
//             "garage", Noun,
//             ".",
//         ), @r###"
//     If     it     is     raining     then     open     the     umbrella     and     close     the     garage  .
//     ╰╯ConditionStart
//                                      ╰──╯Then
//                                                                             ╰─╯And
//     ╰─────────────────────────────╯Condition
//                                      ╰───────────────────────────────────╯TrailingEffect
//                                                                             ╰─────────────────────────────────╯TrailingEffect
//     ╰╯Cond
//            ╰╯Pronoun
//                   ╰╯Verb
//                          ╰─────╯Verb
//                                      ╰──╯CondThen
//                                               ╰──╯Verb
//                                                        ╰─╯Other
//                                                                ╰──────╯Noun
//                                                                             ╰─╯Other
//                                                                                     ╰───╯Verb
//                                                                                               ╰─╯Other
//                                                                                                       ╰────╯Noun
//     "###
//     );
// }

// #[test]
// fn rain_rev() {
//     use POSTag::*;

//     insta::assert_display_snapshot!(
//         tc!(
//             "Open", Verb,
//             " ",
//             "the",
//             " ",
//             "umbrella", Noun,
//             " ",
//             "and", And,
//             " ",
//             "close", Verb,
//             " ",
//             "the",
//             " ",
//             "garage", Noun,
//             " ",
//             "if", Cond,
//             " ",
//             "it", Pronoun,
//             " ",
//             "is", Verb,
//             " ",
//             "raining", Verb,
//             ".",
//         ), @r###"
//     Open     the     umbrella     and     close     the     garage     if     it     is     raining  .
//                                                                        ╰╯ConditionStart
//                                   ╰─╯And
//     ╰───────────────────────────────────────────────────────────────╯LeadingEffect
//                                                                        ╰─────────────────────────────╯Condition
//     ╰──╯Verb
//              ╰─╯Other
//                      ╰──────╯Noun
//                                   ╰─╯Other
//                                           ╰───╯Verb
//                                                     ╰─╯Other
//                                                             ╰────╯Noun
//                                                                        ╰╯Cond
//                                                                               ╰╯Pronoun
//                                                                                      ╰╯Verb
//                                                                                             ╰─────╯Verb
//     "###
//     );
// }

// #[test]
// fn extra_rain() {
//     use POSTag::*;

//     insta::assert_display_snapshot!(
//         tc!(
//             "Open", Verb,
//             " ",
//             "the",
//             " ",
//             "umbrella", Noun,
//             " ",
//             "if", Cond,
//             " ",
//             "it", Pronoun,
//             " ",
//             "is", Verb,
//             " ",
//             "raining", Verb,
//             " ",
//             "and", And,
//             " ",
//             "not",
//             " ",
//             "too",
//             " ",
//             "windy",
//             ".",
//         ), @r###"
//     "###
//     );
// }
