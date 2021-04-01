use super::{LLLine, ToIdx, XDirection, XMatch};

pub trait All {
    type Out;

    fn into_all(self) -> Self::Out;
}

impl<A, B> All for (A, B) {
    type Out = All2<A, B>;

    fn into_all(self) -> Self::Out {
        All2(self.0, self.1)
    }
}

impl<A, B, C> All for (A, B, C) {
    type Out = All3<A, B, C>;

    fn into_all(self) -> Self::Out {
        All3(self.0, self.1, self.2)
    }
}

pub struct All2<A, B>(pub A, pub B);

impl<'l, A: XMatch<'l>, B: XMatch<'l>> XMatch<'l> for All2<A, B> {
    type Out = (A::Out, B::Out);

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        // ╰─╯A(1)
        // ╰─╯A(2)
        // ╰──╯A(3)
        // ╰─╯B(1)
        // ╰─╯B(2)
        // ╰──╯B(3)
        let bs = self.1.go(direction, ll_line);
        self.0
            .go(direction, ll_line)
            .into_iter()
            .flat_map(|(a, a_to_idx)| {
                bs.iter().filter_map(move |(b, b_to_idx)| {
                    if a_to_idx == *b_to_idx {
                        Some(((a, *b), a_to_idx))
                    } else {
                        None
                    }
                })
            })
            .collect()
        // .0 = (&'m A(1), EndIdx(3)); (&'m A(2), EndIdx(3)); (&'m A(3), EndIdx(4))
        // .1 = (&'m B(1), EndIdx(3)); (&'m B(2), EndIdx(3)); (&'m B(3), EndIdx(4))

        // Out[0] = (&'m A(1), &'m B(1)), EndIdx(3)
        // Out[1] = (&'m A(2), &'m B(1)), EndIdx(3)
        // Out[2] = (&'m A(1), &'m B(2)), EndIdx(3)
        // Out[3] = (&'m A(2), &'m B(2)), EndIdx(3)

        // Out[4] = (&'m A(3), &'m B(3)), EndIdx(4)

        // match up the EndIdx values...
    }
}

pub struct All3<A, B, C>(pub A, pub B, pub C);

impl<'l, A: XMatch<'l>, B: XMatch<'l>, C: XMatch<'l>> XMatch<'l> for All3<A, B, C> {
    type Out = (A::Out, B::Out, C::Out);

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, ToIdx)>
    where
        M: XDirection<'l>,
    {
        // match up the EndIdx values...
        let bs = self.1.go(direction, ll_line);
        let cs = self.2.go(direction, ll_line);
        self.0
            .go(direction, ll_line)
            .into_iter()
            .flat_map(|(a, a_to_idx)| {
                let cs_iter = cs.iter();
                bs.iter().flat_map(move |(b, b_to_idx)| {
                    cs_iter.clone().filter_map(move |(c, c_to_idx)| {
                        if &a_to_idx == b_to_idx && &a_to_idx == c_to_idx {
                            Some(((a, *b, *c), a_to_idx))
                        } else {
                            None
                        }
                    })
                })
            })
            .collect()
    }
}
