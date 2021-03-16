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

impl Resolver for ClauseResolver {
    type Attr = Clause;

    fn go(&self, start: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
        start
            .find_start_eq(&ClauseKeyword::ConditionStart)
            .into_iter()
            .flat_map(|start_at_condition| {
                let (cur_matched, _idk) = start_at_condition
                    .match_forwards_until_before_eq_or_until_end_of_line(&ClauseKeyword::Then);

                std::iter::once(cur_matched.finish(Clause::Condition)).chain(
                    cur_matched
                        .start_after()
                        .find_next_start_tag(&TextTag::WORD)
                        .into_iter()
                        .map(|(cur_after_cond, _)| {
                            cur_after_cond
                                .match_forwards_until_before_eq_or_until_end_of_line(
                                    &ClauseKeyword::And,
                                )
                                .0
                                .finish(Clause::TrailingEffect)
                        })
                        .collect::<Vec<_>>()
                        .into_iter(),
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ClauseKeyword {
    /// "and"
    And,
    /// "if", "when"
    ConditionStart,
    /// "then"
    Then,
}

struct ClauseKeywordResolver;

impl Resolver for ClauseKeywordResolver {
    type Attr = ClauseKeyword;

    fn go(&self, cursor: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
        cursor
            .find_start_eq(&POSTag::Cond)
            .into_iter()
            .map(|cur| cur.finish(ClauseKeyword::ConditionStart))
            .chain(
                cursor
                    .find_start_eq(&POSTag::CondThen)
                    .into_iter()
                    .map(|cur| cur.finish(ClauseKeyword::Then)),
            )
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum POSTag {
    QuestionWord,
    Cond,
    CondThen,
    Verb,
    Pronoun,
    Noun,
    Other,
}

fn tc(tokens: &[(&str, &[POSTag])]) -> String {
    let ll_line = create_tokens(
        tokens
            .iter()
            .map(|(text, pos_tags)| {
                InputToken::text(
                    text.to_string(),
                    pos_tags.iter().cloned().map(AnyAttribute::new).collect(),
                )
            })
            .collect(),
        |text| text.encode_utf16().count(),
    )
    .run(&ClauseKeywordResolver {})
    .run(&ClauseResolver {});

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    // ll_line_display.include::<TextTag>();
    ll_line_display.include::<ClauseKeyword>();
    ll_line_display.include::<Clause>();
    ll_line_display.include::<POSTag>();

    format!("{}", ll_line_display)
}

macro_rules! tc {
    ($($text: literal, $($tokens: ident,)*)+) => {
        tc(&[
            $(
                ($text, &[$($tokens,)*]),
            )+
        ])
    }
}

#[test]
fn test_clauses() {
    use POSTag::*;

    insta::assert_display_snapshot!(
        tc!(
            "When", Cond, QuestionWord,
            " ",
            "it", Pronoun,
            " ",
            "rains", Verb,
            " ",
            "then", CondThen,
            " ",
            "it", Pronoun,
            " ",
            "pours", Verb,
            ".",
        ), @r###"
    When     it     rains     then     it     pours  .
    ╰──╯ConditionStart
                              ╰──╯Then
    ╰──────────────────────╯Condition
                              ╰──────────────────────╯TrailingEffect
    ╰──╯QuestionWord
    ╰──╯Cond
             ╰╯Pronoun
                    ╰───╯Verb
                              ╰──╯CondThen
                                       ╰╯Pronoun
                                              ╰───╯Verb
    "###);

    insta::assert_display_snapshot!(
        tc!(
            "If", Cond,
            " ",
            "it", Pronoun,
            " ",
            "is", Verb,
            " ",
            "raining", Verb,
            ",",
            " ",
            "then", CondThen,
            " ",
            "open", Verb,
            " ",
            "the", Other,
            " ",
            "umbrella", Noun,
            ".",
        ), @r###"
    If     it     is     raining  ,     then     open     the     umbrella  .
    ╰╯ConditionStart
                                        ╰──╯Then
    ╰────────────────────────────────╯Condition
                                        ╰───────────────────────────────────╯TrailingEffect
    ╰╯Cond
           ╰╯Pronoun
                  ╰╯Verb
                         ╰─────╯Verb
                                        ╰──╯CondThen
                                                 ╰──╯Verb
                                                          ╰─╯Other
                                                                  ╰──────╯Noun
    "###);
}

#[test]
fn tired() {
    use POSTag::*;

    insta::assert_display_snapshot!(
        tc!(
            "Si", Cond,
            " ",
            "tu", Pronoun,
            " ",
            "es", Verb,
            " ",
            "fatigué", Verb,
            ",", CondThen,
            " ",
            "va", Verb,
            " ",
            "te", Other,
            " ",
            "coucher", Verb,
            ".",
        ), @r###"
    Si     tu     es     fatigué  ,     va     te     coucher  .
    ╰╯ConditionStart
                                  ╰Then
    ╰──────────────────────────╯Condition
                                        ╰──────────────────────╯TrailingEffect
    ╰╯Cond
           ╰╯Pronoun
                  ╰╯Verb
                         ╰─────╯Verb
                                  ╰CondThen
                                        ╰╯Verb
                                               ╰╯Other
                                                      ╰─────╯Verb
    "###);
}
