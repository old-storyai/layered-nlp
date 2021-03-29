use super::*;

pub fn any_of<T: AnyOf>(tuple: T) -> T::Out {
    tuple.into_any()
}

pub fn seq<T: Seq>(tuple: T) -> T::Out {
    tuple.into_seq()
}

pub fn token_text() -> TokenText {
    TokenText(())
}

pub fn attr_eq<A>(attr: &A) -> AttrEq<'_, A> {
    AttrEq { attr }
}

pub fn token_has_any<A: PartialEq>(attrs: &[A]) -> TokenHasAny<'_, A> {
    TokenHasAny { one_of: attrs }
}

pub fn attr<A>() -> Attr<A> {
    Attr(Default::default())
}
