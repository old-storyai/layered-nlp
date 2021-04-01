use super::{LLLine, ToIdx, XDirection, XMatch};

pub struct TokenHasAny<'a, Attr: PartialEq> {
    pub(crate) one_of: &'a [Attr],
}

impl<'l, Attr: PartialEq + 'static> XMatch<'l> for TokenHasAny<'_, Attr> {
    type Out = &'l Attr;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        direction.token_attr_one_of(self.one_of, ll_line)
    }
}
