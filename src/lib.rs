#![doc(
    html_logo_url = "https://raw.githubusercontent.com/storyscript/layered-nlp/main/assets/layered-nlp.svg",
    issue_tracker_base_url = "https://github.com/storyscript/layered-nlp/issues/"
)]

mod create_tokens;
mod ll_line;
mod resolvers;
mod type_bucket;
mod type_id_to_many;

#[cfg(test)]
mod tests;
#[allow(deprecated)]
pub use create_tokens::create_tokens;
pub use create_tokens::{create_line_from_input_tokens, InputToken};

/// Simpler, less featureful version of [create_line_from_input_tokens] which uses utf8 counts as string length ([String::len]).
///
/// Use [create_line_from_input_tokens] to specify custom string length
/// calculation function and to supply custom predefined attributes or
/// custom tokens.
pub fn create_line_from_string<T: AsRef<str>>(input_string: T) -> LLLine {
    let token = InputToken::text(input_string.as_ref().to_string(), Vec::new());
    create_line_from_input_tokens(vec![token], |s| s.len())
}

pub use ll_line::{
    x, FinishWith, LLCursorAssignment, LLLine, LLLineDisplay, LLSelection, Resolver, TextTag,
};
pub use resolvers::TextMatchAssignResolver;
pub use type_bucket::AnyAttribute;

/// Shorthand of [LLLineDisplay::new]
pub fn debug_line<'a>(ll_line: &'a LLLine) -> LLLineDisplay<'a> {
    LLLineDisplay::new(ll_line)
}
