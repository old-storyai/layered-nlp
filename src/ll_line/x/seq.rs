use super::{LLLine, ToIdx, XDirection, XMatch};

pub trait Seq {
    type Out;

    fn into_seq(self) -> Self::Out;
}

impl<A, B> Seq for (A, B) {
    type Out = Seq2<A, B>;

    fn into_seq(self) -> Self::Out {
        Seq2(self.0, self.1)
    }
}

impl<A, B, C> Seq for (A, B, C) {
    type Out = Seq3<A, B, C>;

    fn into_seq(self) -> Self::Out {
        Seq3(self.0, self.1, self.2)
    }
}

pub struct Seq2<A, B>(pub A, pub B);

impl<'l, A: XMatch<'l>, B: XMatch<'l>> XMatch<'l> for Seq2<A, B> {
    type Out = (A::Out, B::Out);

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        self.0
            .go(direction, ll_line)
            .into_iter()
            .flat_map(|(a, to_idx)| {
                Some(
                    self.1
                        .go(&direction.after(to_idx.0, ll_line)?, ll_line)
                        .into_iter()
                        .map(move |(b, to_idx)| ((a, b), to_idx)),
                )
            })
            .flatten()
            .collect()
    }
}

pub struct Seq3<A, B, C>(pub A, pub B, pub C);

impl<'l, A: XMatch<'l>, B: XMatch<'l>, C: XMatch<'l>> XMatch<'l> for Seq3<A, B, C> {
    type Out = (A::Out, B::Out, C::Out);

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        self.0
            .go(direction, ll_line)
            .into_iter()
            .flat_map(|(a, to_idx)| {
                Some(
                    self.1
                        .go(&direction.after(to_idx.0, ll_line)?, ll_line)
                        .into_iter()
                        .flat_map(move |(b, to_idx)| {
                            Some(
                                self.2
                                    .go(&direction.after(to_idx.0, ll_line)?, ll_line)
                                    .into_iter()
                                    .map(move |(c, to_idx)| ((a, b, c), to_idx)),
                            )
                        }),
                )
            })
            .flatten()
            .flatten()
            .collect()
    }
}
