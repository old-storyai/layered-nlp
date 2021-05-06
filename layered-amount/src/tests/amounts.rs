use crate::{Amount, AmountResolver};
use layered_nlp::{create_tokens, InputToken, LLLine, LLLineDisplay};

fn test_setup(sentence: &'static str) -> LLLine {
    create_tokens(
        vec![InputToken::text(sentence.to_string(), Vec::new())],
        |text| text.encode_utf16().count(),
    )
}

#[test]
fn test_amount_simple() {
    let ll_line = test_setup("So I says to him, \"You owes me 50 bucks, prepare y'self to die.\"")
        // run just one resolver
        .run(&AmountResolver::new(
            // french numbers
            vec![' ', '\''],
            ',',
        ));

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<Amount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    So     I     says     to     him  ,     "  You     owes     me     50     bucks  ,     prepare     y'self     to     die  .  "
                                                                       ╰╯Amount(50)
    "###);
}

#[test]
fn test_amount_english() {
    let ll_line = test_setup("First, Paul owed me $1.25, then he owed me $1.35, then he owed me $45,000.24!")
        // run just one resolver
        .run(&AmountResolver::new(
            // french numbers
            vec![',', '_'],
            '.',
        ));

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<Amount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    First  ,     Paul     owed     me     $  1  .  25  ,     then     he     owed     me     $  1  .  35  ,     then     he     owed     me     $  45  ,  000  .  24  !
                                             ╰──────╯Amount(1.25)
                                                                                                ╰──────╯Amount(1.35)
                                                                                                                                                   ╰───────────────╯Amount(45000.24)
    "###);
}

#[test]
fn test_amounts_in_a_list() {
    let ll_line = test_setup("1, 2, 3, 4. 500,000, 600,000, 1 million.")
        // run just one resolver
        .run(&AmountResolver::new(
            // french numbers
            vec![',', '_'],
            '.',
        ));

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<Amount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    1  ,     2  ,     3  ,     4  .     500  ,  000  ,     600  ,  000  ,     1     million  .
    ╰Amount(1)
             ╰Amount(2)
                      ╰Amount(3)
                               ╰Amount(4)
                                        ╰─────────╯Amount(500000)
                                                           ╰─────────╯Amount(600000)
                                                                              ╰Amount(1)
    "###);
}
