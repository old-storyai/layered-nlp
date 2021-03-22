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

    fn go(&self, sel: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        let mut clauses = Vec::new();

        // Start by splitting the line on each `ConditionStart`
        let mut if_blocks = sel
            .split_by(&x::attr_eq(&ClauseKeyword::ConditionStart))
            .into_iter();

        if let Some(first_if_block) = if_blocks.next() {
            if first_if_block == sel {
                // If the first selection is the whole line that means there is no `ConditionStart` in it
                // We probably want to handle this case better in the future
                clauses.push(sel.finish_with_attr(Clause::Independent));
            } else {
                std::iter::once(first_if_block)
                    .chain(if_blocks)
                    .for_each(|if_block_sel| {
                        // If there is a `ConditionStart` in the line we split each selection further by splitting in each `ClauseKeyword`
                        // We get all clauses this way
                        let mut clause_iter = if_block_sel
                            .split_by(&x::attr::<ClauseKeyword>())
                            .into_iter();

                        // The next goal is to determine the kind of each clause
                        // The first clause can only be a `Condition` or `LeadingEffect` based on if it is preceded by a `ConditionStart` or not
                        // The other clauses are either `LeadingEffect` if they are before the `ConditionStart` or `TrailingEffect` if they are after
                        if let Some(sel) = clause_iter.next() {
                            if let Some((sel, _)) = sel
                                .match_first_backwards(&x::attr_eq(&ClauseKeyword::ConditionStart))
                            {
                                clauses.push(sel.finish_with_attr(Clause::Condition));

                                clauses.extend(clause_iter.map(|sel| {
                                    if let Some((sel, _)) =
                                        sel.match_first_backwards(&x::attr::<ClauseKeyword>())
                                    {
                                        sel.finish_with_attr(Clause::TrailingEffect)
                                    } else {
                                        sel.finish_with_attr(Clause::TrailingEffect)
                                    }
                                }));
                            } else {
                                clauses.push(sel.finish_with_attr(Clause::LeadingEffect));

                                clauses.extend(clause_iter.map(|sel| {
                                    if let Some((sel, _)) =
                                        sel.match_first_backwards(&x::attr::<ClauseKeyword>())
                                    {
                                        sel.finish_with_attr(Clause::LeadingEffect)
                                    } else {
                                        sel.finish_with_attr(Clause::LeadingEffect)
                                    }
                                }));
                            }
                        }
                    });
            }
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
