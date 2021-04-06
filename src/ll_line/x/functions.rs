use super::*;
use crate::TextTag;

/// Match if one of the matchers match
pub fn any_of<T: AnyOf>(tuple: T) -> T::Out {
    tuple.into_any()
}

/// Match if all matchers match
pub fn all<T: All>(tuple: T) -> T::Out {
    tuple.into_all()
}

/// Match if all matchers match one after the other
///
/// Example going forward:
///
/// ```txt
/// [ Matcher #1 ]
///               [ Matcher #2 ]
///                             [ Matcher #3 ]
/// ```
pub fn seq<T: Seq>(tuple: T) -> T::Out {
    tuple.into_seq()
}

/// Match single token and provide their text representation
pub fn token_text() -> TokenText {
    TokenText(())
}

/// Match token with `A` attributes equal to `attr`
pub fn attr_eq<A>(attr: &A) -> AttrEq<'_, A> {
    AttrEq { attr }
}

/// Match token with `A` attributes equals to one of `attrs` value
pub fn token_has_any<A: PartialEq>(attrs: &[A]) -> TokenHasAny<'_, A> {
    TokenHasAny { one_of: attrs }
}

/// Match token with `A` attributes
pub fn attr<A>() -> Attr<A> {
    Attr(Default::default())
}

/// Match any number of consecutive spaces.
pub fn whitespace() -> AttrEq<'static, TextTag> {
    attr_eq(&TextTag::SPACE)
}
