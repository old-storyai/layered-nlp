#[test]
fn find_by() {
    use crate::ll_line::{x, LLSelection};
    use crate::tests::test_resolver;

    let split_by_char = |range_sel: LLSelection| {
        range_sel
            .find_by(&x::attr::<char>())
            .into_iter()
            .map(|(sel, _)| sel.finish_with_attr(String::from("here")))
            .collect()
    };

    insta::assert_display_snapshot!(test_resolver("0000aa0000.000aa000.0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  000  aa  000  .  0000  aa  0000
                    ╰"here"
                                     ╰"here"
    "###);
    insta::assert_display_snapshot!(test_resolver("0000aa0000...000aa000..0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  .  .  000  aa  000  .  .  0000  aa  0000
                    ╰"here"
                       ╰"here"
                          ╰"here"
                                           ╰"here"
                                              ╰"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".0000aa0000.000aa000.0000aa0000.", split_by_char), @r###"
    .  0000  aa  0000  .  000  aa  000  .  0000  aa  0000  .
    ╰"here"
                       ╰"here"
                                        ╰"here"
                                                           ╰"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".00.", split_by_char), @r###"
    .  00  .
    ╰"here"
           ╰"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".", split_by_char), @r###"
    .
    ╰"here"
    "###);
}

#[test]
fn find_first_by() {
    use crate::ll_line::{x, LLSelection};
    use crate::tests::test_resolver;

    let split_by_char = |range_sel: LLSelection| {
        range_sel
            .find_first_by(&x::attr::<char>())
            .into_iter()
            .map(|(sel, _)| sel.finish_with_attr(String::from("here")))
            .collect()
    };

    insta::assert_display_snapshot!(test_resolver("0000aa0000.000aa000.0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  000  aa  000  .  0000  aa  0000
                    ╰"here"
    "###);
    insta::assert_display_snapshot!(test_resolver("0000aa0000...000aa000..0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  .  .  000  aa  000  .  .  0000  aa  0000
                    ╰"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".0000aa0000.000aa000.0000aa0000.", split_by_char), @r###"
    .  0000  aa  0000  .  000  aa  000  .  0000  aa  0000  .
    ╰"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".00.", split_by_char), @r###"
    .  00  .
    ╰"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".", split_by_char), @r###"
    .
    ╰"here"
    "###);
}

#[test]
fn split_by() {
    use crate::ll_line::{x, LLSelection};
    use crate::tests::test_resolver;

    let split_by_char = |range_sel: LLSelection| {
        range_sel
            .split_by(&x::attr::<char>())
            .into_iter()
            .map(|sel| sel.finish_with_attr(String::from("here")))
            .collect()
    };

    insta::assert_display_snapshot!(test_resolver("0000aa0000.000aa000.0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  000  aa  000  .  0000  aa  0000
    ╰────────────╯"here"
                       ╰──────────╯"here"
                                        ╰────────────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver("0000aa0000...000aa000..0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  .  .  000  aa  000  .  .  0000  aa  0000
    ╰────────────╯"here"
                             ╰──────────╯"here"
                                                 ╰────────────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".0000aa0000.000aa000.0000aa0000.", split_by_char), @r###"
    .  0000  aa  0000  .  000  aa  000  .  0000  aa  0000  .
       ╰────────────╯"here"
                          ╰──────────╯"here"
                                           ╰────────────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".00.", split_by_char), @r###"
    .  00  .
       ╰╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".", split_by_char), @".");
}

#[test]
fn find_by_seq() {
    use crate::ll_line::{x, LLSelection, TextTag};
    use crate::tests::test_resolver;

    let split_by_char = |range_sel: LLSelection| {
        range_sel
            .find_by(&x::seq((x::attr::<char>(), x::attr_eq(&TextTag::NATN))))
            .into_iter()
            .map(|(sel, _)| sel.finish_with_attr(String::from("here")))
            .collect()
    };

    insta::assert_display_snapshot!(test_resolver("0000aa0000.000aa000.0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  000  aa  000  .  0000  aa  0000
                    ╰────╯"here"
                                     ╰─────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver("0000aa0000...000aa000..0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  .  .  000  aa  000  .  .  0000  aa  0000
                          ╰────╯"here"
                                              ╰─────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".0000aa0000.000aa000.0000aa0000.", split_by_char), @r###"
    .  0000  aa  0000  .  000  aa  000  .  0000  aa  0000  .
    ╰─────╯"here"
                       ╰────╯"here"
                                        ╰─────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".00.", split_by_char), @r###"
    .  00  .
    ╰───╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".", split_by_char), @".");
}

#[test]
fn match_seq_forwards() {
    use crate::ll_line::{x, LLSelection, TextTag};
    use crate::tests::test_resolver;

    let split_by_char = |range_sel: LLSelection| {
        range_sel
            .find_by(&x::attr::<TextTag>())
            .into_iter()
            .map(|(sel, _)| {
                sel.match_forwards(&x::seq((
                    x::attr_eq(&TextTag::NATN),
                    x::attr_eq(&TextTag::WORD),
                )))
            })
            .into_iter()
            .flatten()
            .map(|(sel, _)| sel.finish_with_attr(String::from("here")))
            .collect()
    };

    insta::assert_display_snapshot!(test_resolver("0000aa0000.000aa000.0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  000  aa  000  .  0000  aa  0000
                    ╰────────╯"here"
                                     ╰─────────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver("0000aa0000...000aa000..0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  .  .  000  aa  000  .  .  0000  aa  0000
                          ╰────────╯"here"
                                              ╰─────────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".0000aa0000.000aa000.0000aa0000.", split_by_char), @r###"
    .  0000  aa  0000  .  000  aa  000  .  0000  aa  0000  .
    ╰─────────╯"here"
                       ╰────────╯"here"
                                        ╰─────────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".00.", split_by_char), @".  00  .");
    insta::assert_display_snapshot!(test_resolver(".", split_by_char), @".");
}

#[test]
fn match_seq_backwards() {
    use crate::ll_line::{x, LLSelection, TextTag};
    use crate::tests::test_resolver;

    let split_by_char = |range_sel: LLSelection| {
        range_sel
            .find_by(&x::attr::<TextTag>())
            .into_iter()
            .map(|(sel, _)| {
                sel.match_backwards(&x::seq((
                    x::attr_eq(&TextTag::NATN),
                    x::attr_eq(&TextTag::WORD),
                )))
            })
            .into_iter()
            .flatten()
            .map(|(sel, _)| sel.finish_with_attr(String::from("here")))
            .collect()
    };

    insta::assert_display_snapshot!(test_resolver("0000aa0000.000aa000.0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  000  aa  000  .  0000  aa  0000
          ╰─────────╯"here"
                            ╰────────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver("0000aa0000...000aa000..0000aa0000", split_by_char), @r###"
    0000  aa  0000  .  .  .  000  aa  000  .  .  0000  aa  0000
          ╰─────────╯"here"
                                  ╰────────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".0000aa0000.000aa000.0000aa0000.", split_by_char), @r###"
    .  0000  aa  0000  .  000  aa  000  .  0000  aa  0000  .
             ╰─────────╯"here"
                               ╰────────╯"here"
                                                 ╰─────────╯"here"
    "###);
    insta::assert_display_snapshot!(test_resolver(".00.", split_by_char), @".  00  .");
    insta::assert_display_snapshot!(test_resolver(".", split_by_char), @".");
}
