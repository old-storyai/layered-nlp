mod clause_keyword;
mod clauses;

pub use clause_keyword::{ClauseKeyword, ClauseKeywordResolver};
pub use clauses::{Clause, ClauseResolver};

#[cfg(test)]
mod tests {
    mod clauses;
}
