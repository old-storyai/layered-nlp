use super::{LLLine, ToIdx, XDirection, XMatch};

pub trait AnyOf {
    type Out;

    fn into_any(self) -> Self::Out;
}

impl<A, B> AnyOf for (A, B) {
    type Out = AnyOf2Matcher<A, B>;

    fn into_any(self) -> Self::Out {
        AnyOf2Matcher(self.0, self.1)
    }
}

impl<A, B, C> AnyOf for (A, B, C) {
    type Out = AnyOf3Matcher<A, B, C>;

    fn into_any(self) -> Self::Out {
        AnyOf3Matcher(self.0, self.1, self.2)
    }
}

pub struct AnyOf2Matcher<A, B>(pub A, pub B);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnyOf2<A, B> {
    A(A),
    B(B),
}

impl<'l, A: XMatch<'l>, B: XMatch<'l>> XMatch<'l> for AnyOf2Matcher<A, B> {
    type Out = AnyOf2<A::Out, B::Out>;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        let a = self.0.go(direction, ll_line);

        if !a.is_empty() {
            a.into_iter().map(|(a, idx)| (AnyOf2::A(a), idx)).collect()
        } else {
            self.1
                .go(direction, ll_line)
                .into_iter()
                .map(|(b, idx)| (AnyOf2::B(b), idx))
                .collect()
        }
    }
}

pub struct AnyOf3Matcher<A, B, C>(pub A, pub B, pub C);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnyOf3<A, B, C> {
    A(A),
    B(B),
    C(C),
}

impl<'l, A: XMatch<'l>, B: XMatch<'l>, C: XMatch<'l>> XMatch<'l> for AnyOf3Matcher<A, B, C> {
    type Out = AnyOf3<A::Out, B::Out, C::Out>;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        let a = self.0.go(direction, ll_line);

        if !a.is_empty() {
            a.into_iter().map(|(a, idx)| (AnyOf3::A(a), idx)).collect()
        } else {
            let b = self.1.go(direction, ll_line);

            if !b.is_empty() {
                b.into_iter().map(|(b, idx)| (AnyOf3::B(b), idx)).collect()
            } else {
                self.2
                    .go(direction, ll_line)
                    .into_iter()
                    .map(|(c, idx)| (AnyOf3::C(c), idx))
                    .collect()
            }
        }
    }
}
