#![doc(
    html_logo_url = "https://raw.githubusercontent.com/storyscript/layered-nlp/main/assets/layered-nlp.svg",
    issue_tracker_base_url = "https://github.com/storyscript/layered-nlp/issues/"
)]

mod amounts;

pub use amounts::{Amount, AmountResolver};
pub use rust_decimal;

#[cfg(test)]
mod tests {
    mod amounts;
}
