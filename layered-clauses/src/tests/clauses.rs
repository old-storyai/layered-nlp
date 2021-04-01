use crate::{Clause, ClauseKeyword, ClauseKeywordResolver, ClauseResolver};
use layered_nlp::{create_tokens, InputToken, LLLine, LLLineDisplay};
use layered_part_of_speech::{POSTagResolver, Tag};

fn test_setup(sentence: &'static str) -> LLLine {
    create_tokens(
        vec![InputToken::text(sentence.to_string(), Vec::new())],
        |text| text.encode_utf16().count(),
    )
}

#[test]
fn test_clauses() {
    let ll_line = test_setup("When it rains then it pours.")
        .run(&ClauseKeywordResolver::new(
            &["if", "when"],
            &["and"],
            &["then"],
        ))
        .run(&ClauseResolver::default());

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<ClauseKeyword>();
    ll_line_display.include::<Clause>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    When     it     rains     then     it     pours  .
    ╰──╯ConditionStart
                              ╰──╯Then
    ╰──────────────────────╯Condition
                              ╰───────────────────╯TrailingEffect
    "###);
}

#[test]
fn test_clauses_comma() {
    let ll_line = test_setup("If it is raining, open your umbrella.")
        .run(&POSTagResolver::default())
        .run(&ClauseKeywordResolver::new(
            &["if", "when"],
            &["and"],
            &["then"],
        ))
        .run(&ClauseResolver::default());

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<ClauseKeyword>();
    ll_line_display.include::<Clause>();
    ll_line_display.include::<Tag>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    If     it     is     raining  ,     open     your     umbrella  .
    ╰╯ConditionStart
    ╰──────────────────────────╯Condition
                                     ╰───────────────────────────╯TrailingEffect
    ╰╯Noun
    ╰╯Conjunction
           ╰╯Pronoun
           ╰╯Noun
           ╰╯Determiner
           ╰╯Adjective
                  ╰╯Verb
                  ╰╯Noun
                         ╰─────╯Verb
                         ╰─────╯Noun
                                        ╰──╯Verb
                                        ╰──╯Noun
                                        ╰──╯Adjective
                                                 ╰──╯Pronoun
                                                 ╰──╯Determiner
                                                          ╰──────╯Verb
                                                          ╰──────╯Noun
    "###);
}

#[test]
fn tired() {
    let ll_line = test_setup("Si tu es fatigué, va te coucher.")
        .run(&ClauseKeywordResolver::new(&["si"], &["et"], &["alors"]))
        .run(&ClauseResolver::default());

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<ClauseKeyword>();
    ll_line_display.include::<Clause>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    Si     tu     es     fatigué  ,     va     te     coucher  .
    ╰╯ConditionStart
    ╰──────────────────────────╯Condition
                                     ╰──────────────────────╯TrailingEffect
    "###);
}

#[test]
fn tired_rev() {
    let ll_line = test_setup("Va te coucher si tu es fatigué.")
        .run(&ClauseKeywordResolver::new(&["si"], &["et"], &["alors"]))
        .run(&ClauseResolver::default());

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<ClauseKeyword>();
    ll_line_display.include::<Clause>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    Va     te     coucher     si     tu     es     fatigué  .
                              ╰╯ConditionStart
    ╰──────────────────────╯LeadingEffect
                              ╰──────────────────────────╯Condition
    "###);
}

#[test]
fn rain() {
    let ll_line =
        test_setup("If it is raining then open the umbrella and close the garage and the door.")
            .run(&ClauseKeywordResolver::new(
                &["if", "when"],
                &["and"],
                &["then"],
            ))
            .run(&ClauseResolver::default());

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<ClauseKeyword>();
    ll_line_display.include::<Clause>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    If     it     is     raining     then     open     the     umbrella     and     close     the     garage     and     the     door  .
    ╰╯ConditionStart
                                     ╰──╯Then
                                                                            ╰─╯And
                                                                                                                 ╰─╯And
    ╰─────────────────────────────╯Condition
                                     ╰───────────────────────────────────╯TrailingEffect
                                                                            ╰─────────────────────────────────╯TrailingEffect
                                                                                                                 ╰──────────────────╯TrailingEffect
    "###
    );
}

#[test]
fn rain_rev() {
    let ll_line =
        test_setup("Open the umbrella and close the garage and the door if it is raining.")
            .run(&ClauseKeywordResolver::new(
                &["if", "when"],
                &["and"],
                &["then"],
            ))
            .run(&ClauseResolver::default());

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<ClauseKeyword>();
    ll_line_display.include::<Clause>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    Open     the     umbrella     and     close     the     garage     and     the     door     if     it     is     raining  .
                                  ╰─╯And
                                                                       ╰─╯And
                                                                                                ╰╯ConditionStart
    ╰──────────────────────────╯LeadingEffect
                                  ╰─────────────────────────────────╯LeadingEffect
                                                                       ╰─────────────────────╯LeadingEffect
                                                                                                ╰──────────────────────────╯Condition
    "###
    );
}

#[test]
fn extra_rain() {
    let ll_line = test_setup("Open the umbrella if it is raining and not too windy.")
        .run(&ClauseKeywordResolver::new(
            &["if", "when"],
            &["and"],
            &["then"],
        ))
        .run(&ClauseResolver::default());

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<ClauseKeyword>();
    ll_line_display.include::<Clause>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    Open     the     umbrella     if     it     is     raining     and     not     too     windy  .
                                  ╰╯ConditionStart
                                                                   ╰─╯And
    ╰──────────────────────────╯LeadingEffect
                                  ╰─────────────────────────────╯Condition
                                                                   ╰───────────────────────────╯TrailingEffect
    "###
    );
}

#[test]
fn no_keyword() {
    let ll_line = test_setup("Open the umbrella.")
        .run(&ClauseKeywordResolver::new(
            &["if", "when"],
            &["and"],
            &["then"],
        ))
        .run(&ClauseResolver::default());

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<ClauseKeyword>();
    ll_line_display.include::<Clause>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    Open     the     umbrella  .
    ╰──────────────────────────╯Independent
    "###
    );
}
