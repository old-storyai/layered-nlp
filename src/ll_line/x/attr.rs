use super::{LLLine, ToIdx, XDirection, XMatch};

pub struct Attr<Attr>(pub(crate) std::marker::PhantomData<Attr>);

impl<'l, A: 'static> XMatch<'l> for Attr<A> {
    type Out = &'l A;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        direction.attr::<A>(ll_line)
    }
}
