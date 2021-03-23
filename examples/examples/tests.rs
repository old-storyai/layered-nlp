use layered_nlp::{create_tokens, InputToken, LLLineDisplay};
use layered_part_of_speech::*;

fn main() {
    let ll_line = create_tokens(
        vec![InputToken::Text {
            text: "Don't step on the broken glass.".to_string(),
            attrs: Vec::new(),
        }],
        |text| text.encode_utf16().count(),
    );

    let ll_line = ll_line.run(&POSTagResolver {});

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<Tag>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    Don't     step     on     the     broken     glass  .
              ╰──╯Noun(Other)
              ╰──╯Verb
                              ╰─╯Determiner
    "###);
}
