pub use wiktionary_part_of_speech_extract::Tag;

use layered_nlp::{x, LLCursorAssignment, LLSelection, Resolver, TextTag};
use wiktionary_part_of_speech_extract::{TagSet, ENGLISH_TAG_LOOKUP};

#[derive(Default)]
pub struct POSTagResolver(());

impl Resolver for POSTagResolver {
    type Attr = Tag;

    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        selection
            .find_by(&x::all((x::attr_eq(&TextTag::WORD), x::token_text())))
            .into_iter()
            .flat_map(|(selection, (_, word))| {
                ENGLISH_TAG_LOOKUP
                    .get(&word.to_lowercase())
                    .unwrap_or_else(|| TagSet::of(&[Tag::Noun]))
                    .tags()
                    .map(move |tag| selection.finish_with_attr(tag))
            })
            .collect()
    }
}
