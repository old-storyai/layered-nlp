#![allow(dead_code, unused_variables)]
use super::*;

impl super::LLCursorStart {
    fn match_into<T: MatchTuple>(&self) -> impl IntoIterator<Item = T> {
        vec![]
    }

    fn test_match_into(&self) {
        for mat in self.match_into() {
            if let (StartAnywhere(cursor), TextTag::PUNC, Any(TextTag::WORD, tags)) = mat {
                // hmmm
            }
        }
    }
}

trait MatchItem {
    type Out;
}

struct StartAnywhere(pub LLCursor);

impl MatchItem for StartAnywhere {
    type Out = StartAnywhere;
}

struct Any<T: MatchItem>(pub T, pub Vec<T::Out>);

impl<T: MatchItem> MatchItem for Any<T> {
    type Out = Any<T>;
}

trait MatchTuple {}

impl MatchItem for TextTag {
    type Out = String;
}

impl<A: MatchItem, B: MatchItem, C: MatchItem> MatchTuple for (A, B, C) {}

impl<A: MatchItem, B: MatchItem> MatchTuple for (A, B) {}
