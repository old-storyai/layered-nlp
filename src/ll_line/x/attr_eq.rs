use super::{LLLine, ToIdx, XDirection, XMatch};

pub struct AttrEq<'a, Attr> {
    pub(crate) attr: &'a Attr,
}

impl<'l, Attr: PartialEq + 'static> XMatch<'l> for AttrEq<'_, Attr> {
    type Out = ();

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        direction.attr_eq(self.attr, ll_line)
    }
}
