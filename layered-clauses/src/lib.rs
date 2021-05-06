#![doc(
    html_logo_url = "https://raw.githubusercontent.com/storyscript/layered-nlp/main/assets/layered-nlp.svg",
    issue_tracker_base_url = "https://github.com/storyscript/layered-nlp/issues/"
)]

mod clause_keyword;
mod clauses;

pub use clause_keyword::{ClauseKeyword, ClauseKeywordResolver};
pub use clauses::{Clause, ClauseResolver};

#[cfg(test)]
mod tests {
    mod clauses;
}
