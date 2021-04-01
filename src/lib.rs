mod create_tokens;
mod ll_line;
mod resolvers;
mod type_bucket;
mod type_id_to_many;

#[cfg(test)]
mod tests;

pub use create_tokens::{create_tokens, InputToken};
pub use ll_line::{
    x, FinishWith, LLCursorAssignment, LLLine, LLLineDisplay, LLSelection, Resolver, TextTag,
};
pub use resolvers::TextMatchAssignResolver;
pub use type_bucket::AnyAttribute;

#[derive(Debug, PartialEq, Clone)]
pub struct ClauseRange {
    pub start: usize,
    pub end: usize,
    pub is_conjunction: bool,
}
