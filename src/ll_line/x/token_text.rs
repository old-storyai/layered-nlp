use super::{LLLine, ToIdx, XDirection, XMatch};

pub struct TokenText(pub(crate) ());

impl<'l> XMatch<'l> for TokenText {
    type Out = &'l str;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        direction.text_token(ll_line).into_iter().collect()
    }
}
