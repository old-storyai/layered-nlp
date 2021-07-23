use super::ClauseKeyword;
use layered_nlp::{x, LLCursorAssignment, LLSelection, Resolver, TextTag};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Clause {
    LeadingEffect,
    TrailingEffect,
    Condition,
    Independent,
}

#[derive(Default)]
pub struct ClauseResolver {
    // hmmm... configuration?
}

impl Resolver for ClauseResolver {
    type Attr = Clause;

    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        let mut condition_clause_found = false;

        let clauses = selection.split_by(&x::any_of((
            x::attr::<ClauseKeyword>(),
            x::attr_eq(&TextTag::PUNC),
        )));

        if clauses.len() == 1 {
            // If the first selection is the whole line that means there is no `ConditionStart` in it
            // We probably want to handle this case better in the future
            if let Some(trimmed_clause_selection) = clauses[0].trim(&x::whitespace()) {
                if let Some((_, ClauseKeyword::ConditionStart)) =
                    clauses[0].match_first_backwards(&x::attr::<ClauseKeyword>())
                {
                    vec![trimmed_clause_selection.finish_with_attr(Clause::Condition)]
                } else {
                    vec![trimmed_clause_selection.finish_with_attr(Clause::Independent)]
                }
            } else {
                Vec::new()
            }
        } else {
            clauses
                .into_iter()
                .filter_map(|clause_selection| {
                    let trimmed_clause_selection = clause_selection.trim(&x::whitespace())?;

                    if let Some((_, clause_keyword)) =
                        clause_selection.match_first_backwards(&x::attr::<ClauseKeyword>())
                    {
                        match clause_keyword {
                            ClauseKeyword::ConditionStart => {
                                condition_clause_found = true;

                                Some(trimmed_clause_selection.finish_with_attr(Clause::Condition))
                            }
                            ClauseKeyword::Then => Some(
                                trimmed_clause_selection.finish_with_attr(Clause::TrailingEffect),
                            ),
                            ClauseKeyword::And => Some(trimmed_clause_selection.finish_with_attr(
                                if condition_clause_found {
                                    Clause::TrailingEffect
                                } else {
                                    Clause::LeadingEffect
                                },
                            )),
                        }
                    } else if condition_clause_found {
                        Some(trimmed_clause_selection.finish_with_attr(Clause::TrailingEffect))
                    } else {
                        Some(trimmed_clause_selection.finish_with_attr(Clause::LeadingEffect))
                    }
                })
                .collect()
        }
    }
}
