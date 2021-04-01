use super::{LLCursorAssignment, LLSelection};

pub trait FinishWith<T> {
    fn finish_with<Attr, F: Fn(T) -> Attr>(self, f: F) -> Vec<LLCursorAssignment<Attr>>;
}

impl<T, I: IntoIterator<Item = (LLSelection, T)>> FinishWith<T> for I {
    fn finish_with<Attr, F: Fn(T) -> Attr>(self, f: F) -> Vec<LLCursorAssignment<Attr>> {
        self.into_iter()
            .map(|(selection, t)| selection.finish_with_attr(f(t)))
            .collect()
    }
}
