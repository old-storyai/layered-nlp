use super::*;
use std::marker::PhantomData;

pub struct AttrMatcher<T> {
    cursors: std::vec::IntoIter<LLCursor>,
    phantom: PhantomData<T>,
}

//

impl<T> AttrMatcher<T> {
    fn finish<U: Clone>(self, value: U) -> Vec<LLCursorAssignment<U>> {
        self.cursors
            .into_iter()
            .map(|cursor| cursor.finish_with_attr(value.clone()))
            .collect()
    }
}

impl LLCursorStart {
    pub fn for_each_attr_eq<T: 'static + std::fmt::Debug + PartialEq>(
        &self,
        attr: &T,
    ) -> AttrMatcher<T> {
        AttrMatcher {
            cursors: self
                .ll_line
                .attrs
                .values
                .iter()
                .filter_map(|(&(start_idx, end_idx), bucket)| {
                    let attrs = bucket.get::<T>();
                    if !attrs.is_empty() {
                        Some(
                            attrs
                                .iter()
                                .filter(|current_attr| *current_attr == attr)
                                .map(move |_| LLCursor {
                                    start_idx,
                                    end_idx,
                                    ll_line: self.ll_line.clone(),
                                }),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<_>>()
                .into_iter(),
            phantom: PhantomData,
        }
    }
}
