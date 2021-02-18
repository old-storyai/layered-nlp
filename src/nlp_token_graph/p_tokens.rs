use std::ops::Deref;

use storycore::edit::EditToken;

pub use p_token::{PToken, PTokenKind};

mod p_token;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PTokenAt(usize);

/// End bound (exclusive)
#[derive(PartialEq, Eq)]
pub enum PTokenRangeTo {
    Before(PTokenAt),
    End,
}

/// Immutable container of [PTokenRange]
pub struct PTokenRange(Vec<p_token::PToken>);

impl PTokenRange {
    pub fn from_token_line(tokens: &[EditToken]) -> Self {
        unimplemented!()
    }
}

impl Deref for PTokenRange {
    type Target = [p_token::PToken];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for PTokenRangeTo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PTokenRangeTo::Before(a), PTokenRangeTo::Before(b)) => a.partial_cmp(b),
            (PTokenRangeTo::End, PTokenRangeTo::Before(_)) => Some(std::cmp::Ordering::Greater),
            (PTokenRangeTo::Before(_), PTokenRangeTo::End) => Some(std::cmp::Ordering::Less),
            (PTokenRangeTo::End, PTokenRangeTo::End) => Some(std::cmp::Ordering::Equal),
        }
    }
}
