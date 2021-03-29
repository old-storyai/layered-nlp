use layered_nlp::{create_tokens, InputToken, LLLineDisplay};
use layered_part_of_speech::*;

fn main() {
    let ll_line = create_tokens(
        vec![InputToken::text(
            "Don't step on the broken glass and the tablesaw in Paris.".to_string(),
            Vec::new(),
        )],
        |text| text.encode_utf16().count(),
    );

    let ll_line = ll_line.run(&POSTagResolver::default());

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<Tag>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    Don't     step     on     the     broken     glass     and     the     tablesaw     in     Paris  .
    ╰───╯Verb
    ╰───╯Noun
    ╰───╯Interjection
              ╰──╯Verb
              ╰──╯Noun
                       ╰╯Verb
                       ╰╯Preposition
                       ╰╯Noun
                       ╰╯Adverb
                       ╰╯Adjective
                              ╰─╯Preposition
                              ╰─╯Adverb
                                      ╰────╯Verb
                                      ╰────╯Adjective
                                                 ╰───╯Verb
                                                 ╰───╯Noun
                                                           ╰─╯Verb
                                                           ╰─╯Noun
                                                           ╰─╯Conjunction
                                                                   ╰─╯Preposition
                                                                   ╰─╯Adverb
                                                                           ╰──────╯Noun
                                                                                        ╰╯Verb
                                                                                        ╰╯Preposition
                                                                                        ╰╯Noun
                                                                                        ╰╯Adverb
                                                                                        ╰╯Adjective
                                                                                               ╰───╯Noun
    "###);
}
