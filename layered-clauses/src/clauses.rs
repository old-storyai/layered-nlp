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
        let mut clauses = Vec::new();

        // Start by splitting the line on each `ConditionStart`
        let if_blocks = selection.split_by(&x::attr_eq(&ClauseKeyword::ConditionStart));

        if if_blocks.first() == Some(&selection) {
            // If the first selection is the whole line that means there is no `ConditionStart` in it
            // We probably want to handle this case better in the future
            if let Some(selection) = selection.trim(&x::whitespace()) {
                clauses.push(selection.finish_with_attr(Clause::Independent));
            }
        } else {
            // `filter_map` is used to be able to use `?`
            if_blocks
                .into_iter()
                .filter_map(|if_block_sel| {
                    // If there is a `ConditionStart` in the line we split each selection further by splitting in each `ClauseKeyword`
                    // We get all clauses this way
                    let mut clause_iter = if_block_sel
                        .split_by(&x::any_of((
                            x::attr::<ClauseKeyword>(),
                            x::attr_eq(&TextTag::PUNC),
                        )))
                        .into_iter();

                    // The next goal is to determine the kind of each clause
                    // The first clause can only be a `Condition` or `LeadingEffect` based on if it is preceded by a `ConditionStart` or not
                    // The other clauses are either `LeadingEffect` if they are before the `ConditionStart` or `TrailingEffect` if they are after
                    if let Some(first_clause_sel) = clause_iter.next() {
                        if first_clause_sel
                            .match_first_backwards(&x::attr_eq(&ClauseKeyword::ConditionStart))
                            .is_some()
                        {
                            clauses.push(
                                first_clause_sel
                                    .trim(&x::whitespace())?
                                    .finish_with_attr(Clause::Condition),
                            );

                            clauses.extend(clause_iter.filter_map(|clause_sel| {
                                Some(
                                    clause_sel
                                        .trim(&x::whitespace())?
                                        .finish_with_attr(Clause::TrailingEffect),
                                )
                            }));
                        } else {
                            clauses.extend(
                                std::iter::once(first_clause_sel)
                                    .chain(clause_iter)
                                    .filter_map(|clause_sel| {
                                        Some(
                                            clause_sel
                                                .trim(&x::whitespace())?
                                                .finish_with_attr(Clause::LeadingEffect),
                                        )
                                    }),
                            );
                        }
                    }

                    Some(())
                })
                .for_each(drop);
        }

        clauses
    }
}
