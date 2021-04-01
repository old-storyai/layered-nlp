mod currency_amount;
mod ll_selection;
mod tokenizing;

use crate::create_tokens::*;
use crate::ll_line::{LLCursorAssignment, LLLine, LLLineDisplay, LLSelection, Resolver, TextTag};

pub(crate) fn test_line(input: &str) -> LLLine {
    create_tokens(
        vec![InputToken::Text {
            text: input.to_string(),
            attrs: Vec::new(),
        }],
        |text| text.encode_utf16().count(),
    )
}

pub(crate) fn test_resolver<F>(s: &str, f: F) -> String
where
    F: Fn(LLSelection) -> Vec<LLCursorAssignment<String>>,
{
    let ll_line = crate::tests::test_line(s).run(&TestResolver(f));

    // display insta test
    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<String>();

    format!("{}", &ll_line_display)
}

struct TestResolver<F>(F)
where
    F: Fn(LLSelection) -> Vec<LLCursorAssignment<String>>;

impl<F> Resolver for TestResolver<F>
where
    F: Fn(LLSelection) -> Vec<LLCursorAssignment<String>>,
{
    type Attr = String;

    fn go(&self, start: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        self.0(start)
    }
}
