use super::*;

/// `selection.as_tokens(&'a self) -> Vec<AsToken<'a>>`
pub enum AsToken<'a> {
    Text(&'a str, TokenAttrs<'a>),
    Value(TokenAttrs<'a>),
}

pub struct TokenAttrs<'a> {
    token_idx: usize,
    attrs: &'a LLLineAttrs,
}

impl<'a> TokenAttrs<'a> {
    pub fn get_attr<T: 'static>(&'a self) -> Vec<&'a T> {
        todo!()
    }
}
