use layered_nlp::{x, LLCursorAssignment, LLSelection, Resolver, TextTag};
use std::ops::Range;

include!(concat!(env!("OUT_DIR"), "/tags.rs"));

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Noun {
    Other,
    Pronoun,
    Possessive,
    RelativePronoun,
    Action,
    Actor,
    Service,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tag {
    Noun(Noun),
    Verb, //TODO: Form (Gerund, Infinitive, Tense, etc.)
    Adjective,
    Adverb,
    Determiner,
    Conjunction,
    Comma,
    Point,
    Colon,
    Semicolon,
    ExclamationMark,
    QuestionMark,
    Quote,
    OpenParenthesis,
    CloseParenthesis,
}

#[derive(Default)]
pub struct POSTagResolver(());

impl Resolver for POSTagResolver {
    type Attr = Tag;

    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        static FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/words.fst"));

        let map = fst::Map::new(FST).unwrap();

        selection
            .find_by(&(x::attr_eq(&TextTag::WORD), x::token_text()))
            .into_iter()
            .flat_map(|(selection, (_, word))| {
                map.get(word.to_lowercase()).map(move |tag_idx| {
                    TAGS[tag_idx as usize]
                        .iter()
                        .copied()
                        .map(move |tag| selection.finish_with_attr(tag))
                })
            })
            .flatten()
            .collect()
    }
}
