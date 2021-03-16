use insta;
use rust_decimal::Decimal;

use crate::create_tokens::*;
use crate::ll_line::{LLCursorAssignment, LLCursorStart, LLLine, LLLineDisplay, Resolver, TextTag};

fn split_input(input: &str) -> String {
    format!("{:#?}", test_line(input).ll_tokens())
}

pub(crate) fn test_line(input: &str) -> LLLine {
    create_tokens(
        vec![InputToken::Text {
            text: input.to_string(),
            attrs: Vec::new(),
        }],
        |text| text.encode_utf16().count(),
    )
}

mod clauses;
mod currency_amount;
mod tokenizing;
