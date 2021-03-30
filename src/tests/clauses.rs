use crate::create_tokens::InputToken;
use crate::ll_line::x;
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

    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        let mut clauses = Vec::new();

        let mut found_cond = false;

        selection
            .split_by(&x::attr::<ClauseKeyword>())
            .into_iter()
            .for_each(|clause_sel| {
                if let Some((clause_and_keyword_sel, keyword)) = clause_sel.match_first_backwards(&x::attr::<ClauseKeyword>())
                {
                    match keyword {
                        ClauseKeyword::And => {
                            clauses.push(clause_and_keyword_sel.finish_with_attr(if found_cond {
                                Clause::TrailingEffect
                            } else {
                                Clause::LeadingEffect
                            }));
                        }
                        ClauseKeyword::ConditionStart => {
                            found_cond = true;

                            clauses.push(clause_and_keyword_sel.finish_with_attr(Clause::Condition));
                        }
                        ClauseKeyword::Then => {
                            clauses.push(clause_and_keyword_sel.finish_with_attr(Clause::TrailingEffect));
                        }
                    }
                } else {
                    clauses.push(clause_sel.finish_with_attr(Clause::LeadingEffect));
                }
            });

        if !found_cond {
            clauses = vec![selection.finish_with_attr(Clause::Independent)];
        }

        clauses
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

    fn go(&self, sel: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        sel.find_by(&x::attr_eq(&POSTag::Cond))
            .into_iter()
            .map(|(sel, _)| sel.finish_with_attr(ClauseKeyword::ConditionStart))
            .chain(
                sel.find_by(&x::attr_eq(&POSTag::CondThen))
                    .into_iter()
                    .map(|(sel, _)| sel.finish_with_attr(ClauseKeyword::Then)),
            )
            .chain(
                sel.find_by(&x::attr_eq(&POSTag::And))
                    .into_iter()
                    .map(|(sel, _)| sel.finish_with_attr(ClauseKeyword::And)),
            )
            .collect()
    }
}

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
            "fatigué", Adjective,
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
                                  ╰────────────────────────────╯TrailingEffect
    ╰╯Cond
           ╰╯Pronoun
                  ╰╯Verb
                         ╰─────╯Adjective
                                  ╰CondThen
                                        ╰╯Verb
                                               ╰╯Other
                                                      ╰─────╯Verb
    "###);
}

#[test]
fn tired_rev() {
    use POSTag::*;

    insta::assert_display_snapshot!(
        tc!(
            "Va", Verb,
            " ",
            "te", Other,
            " ",
            "coucher", Verb,
            ",",
            " ",
            "si", Cond,
            " ",
            "tu", Pronoun,
            " ",
            "es", Verb,
            " ",
            "fatigué", Adjective,
            ".",
        ), @r###"
    Va     te     coucher  ,     si     tu     es     fatigué  .
                                 ╰╯ConditionStart
    ╰─────────────────────────╯LeadingEffect
                                 ╰─────────────────────────────╯Condition
    ╰╯Verb
           ╰╯Other
                  ╰─────╯Verb
                                 ╰╯Cond
                                        ╰╯Pronoun
                                               ╰╯Verb
                                                      ╰─────╯Adjective
    "###);
}

#[test]
fn rain() {
    use POSTag::*;

    insta::assert_display_snapshot!(
        tc!(
            "If", Cond,
            " ",
            "it", Pronoun,
            " ",
            "is", Verb,
            " ",
            "raining", Verb,
            " ",
            "then", CondThen,
            " ",
            "open", Verb,
            " ",
            "the",
            " ",
            "umbrella", Noun,
            " ",
            "and", And,
            " ",
            "close", Verb,
            " ",
            "the",
            " ",
            "garage", Noun,
            " ",
            "and", And,
            " ",
            "the",
            " ",
            "door", Noun,
            ".",
        ), @r###"
    If     it     is     raining     then     open     the     umbrella     and     close     the     garage     and     the     door  .
    ╰╯ConditionStart
                                     ╰──╯Then
                                                                            ╰─╯And
                                                                                                                 ╰─╯And
    ╰─────────────────────────────╯Condition
                                     ╰───────────────────────────────────╯TrailingEffect
                                                                            ╰─────────────────────────────────╯TrailingEffect
                                                                                                                 ╰─────────────────────╯TrailingEffect
    ╰╯Cond
           ╰╯Pronoun
                  ╰╯Verb
                         ╰─────╯Verb
                                     ╰──╯CondThen
                                              ╰──╯Verb
                                                               ╰──────╯Noun
                                                                            ╰─╯And
                                                                                    ╰───╯Verb
                                                                                                      ╰────╯Noun
                                                                                                                 ╰─╯And
                                                                                                                                 ╰──╯Noun
    "###
    );
}

#[test]
fn rain_rev() {
    use POSTag::*;

    insta::assert_display_snapshot!(
        tc!(
            "Open", Verb,
            " ",
            "the",
            " ",
            "umbrella", Noun,
            " ",
            "and", And,
            " ",
            "close", Verb,
            " ",
            "the",
            " ",
            "garage", Noun,
            " ",
            "and", And,
            " ",
            "close", Verb,
            " ",
            "the",
            " ",
            "garage", Noun,
            " ",
            "if", Cond,
            " ",
            "it", Pronoun,
            " ",
            "is", Verb,
            " ",
            "raining", Verb,
            ".",
        ), @r###"
    Open     the     umbrella     and     close     the     garage     and     close     the     garage     if     it     is     raining  .
                                                                                                            ╰╯ConditionStart
                                  ╰─╯And
                                                                       ╰─╯And
    ╰──────────────────────────╯LeadingEffect
                                  ╰─────────────────────────────────╯LeadingEffect
                                                                       ╰─────────────────────────────────╯LeadingEffect
                                                                                                            ╰─────────────────────────────╯Condition
    ╰──╯Verb
                     ╰──────╯Noun
                                  ╰─╯And
                                          ╰───╯Verb
                                                            ╰────╯Noun
                                                                       ╰─╯And
                                                                               ╰───╯Verb
                                                                                                 ╰────╯Noun
                                                                                                            ╰╯Cond
                                                                                                                   ╰╯Pronoun
                                                                                                                          ╰╯Verb
                                                                                                                                 ╰─────╯Verb
    "###
    );
}

#[test]
fn extra_rain() {
    use POSTag::*;

    insta::assert_display_snapshot!(
        tc!(
            "Open", Verb,
            " ",
            "the",
            " ",
            "umbrella", Noun,
            " ",
            "if", Cond,
            " ",
            "it", Pronoun,
            " ",
            "is", Verb,
            " ",
            "raining", Verb,
            " ",
            "and", And,
            " ",
            "not",
            " ",
            "too",
            " ",
            "windy",
            ".",
        ), @r###"
    Open     the     umbrella     if     it     is     raining     and     not     too     windy  .
                                  ╰╯ConditionStart
                                                                   ╰─╯And
    ╰──────────────────────────╯LeadingEffect
                                  ╰─────────────────────────────╯Condition
                                                                   ╰──────────────────────────────╯TrailingEffect
    ╰──╯Verb
                     ╰──────╯Noun
                                  ╰╯Cond
                                         ╰╯Pronoun
                                                ╰╯Verb
                                                       ╰─────╯Verb
                                                                   ╰─╯And
    "###
    );
}

#[test]
fn no_keyword() {
    use POSTag::*;

    insta::assert_display_snapshot!(
        tc!(
            "Open", Verb,
            " ",
            "the",
            " ",
            "umbrella", Noun,
            ".",
        ), @r###"
    Open     the     umbrella  .
    ╰──────────────────────────╯Independent
    ╰──╯Verb
                     ╰──────╯Noun
    "###
    );
}
