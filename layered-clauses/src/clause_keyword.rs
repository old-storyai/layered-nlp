use layered_nlp::{x, LLCursorAssignment, LLSelection, Resolver};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClauseKeyword {
    /// "if", "when"
    ConditionStart,
    /// "then"
    Then,
    /// "and"
    And,
}

pub struct ClauseKeywordResolver {
    cond_start: Vec<&'static str>,
    and: Vec<&'static str>,
    then: Vec<&'static str>,
}

impl ClauseKeywordResolver {
    pub fn new(cond_start: &[&'static str], and: &[&'static str], then: &[&'static str]) -> Self {
        ClauseKeywordResolver {
            cond_start: cond_start.to_vec(),
            and: and.to_vec(),
            then: then.to_vec(),
        }
    }
}

impl Resolver for ClauseKeywordResolver {
    type Attr = ClauseKeyword;

    fn go(&self, sel: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        sel.find_by(&x::token_text())
            .into_iter()
            .flat_map(|(sel, text)| {
                let text = text.to_lowercase();

                Some(
                    sel.finish_with_attr(if self.cond_start.contains(&text.as_str()) {
                        ClauseKeyword::ConditionStart
                    } else if self.then.contains(&text.as_str()) {
                        ClauseKeyword::Then
                    } else if self.and.contains(&text.as_str()) {
                        ClauseKeyword::And
                    } else {
                        return None;
                    }),
                )
            })
            .collect()
    }
}
