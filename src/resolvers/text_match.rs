use std::collections::HashMap;
use std::fmt::Debug;

use crate::{x, Resolver};

/// Useful for name matching
pub struct TextMatchAssignResolver<T: Clone> {
    case_sensitive: bool,
    // $ tokens $$ text with spaces $$$ regex?
    lookup: HashMap<String, Vec<T>>,
}

impl<T: Clone> TextMatchAssignResolver<T> {
    pub fn new(lookup: HashMap<String, Vec<T>>) -> Self {
        TextMatchAssignResolver {
            case_sensitive: true,
            lookup,
        }
    }
    pub fn new_case_insensitive(lookup: HashMap<String, Vec<T>>) -> Self {
        TextMatchAssignResolver {
            case_sensitive: false,
            lookup: lookup
                .into_iter()
                .map(|(key, val)| (key.to_lowercase(), val))
                .collect(),
        }
    }
}

impl<T: Debug + Clone + 'static> Resolver for TextMatchAssignResolver<T> {
    type Attr = T;

    fn go(&self, selection: crate::LLSelection) -> Vec<crate::LLCursorAssignment<Self::Attr>> {
        selection
            .find_by(&x::token_text())
            .into_iter()
            .flat_map(|(selection, text)| {
                if self.case_sensitive {
                    self.lookup.get(text)
                } else {
                    self.lookup.get(&text.to_lowercase())
                }
                .map(|values| {
                    values
                        .iter()
                        .cloned()
                        .map(move |attr: T| selection.finish_with_attr(attr))
                })
            })
            .flatten()
            .collect()
    }
}

#[test]
fn test() {
    use crate::{create_tokens, InputToken, LLLineDisplay};

    #[derive(Debug, Clone)]
    enum Service {
        Slack,
        Algolia,
        Magic,
        Wolfram,
    }

    let ll_line = create_tokens(
        vec![
            InputToken::text("when Slack hears a message in #general".to_string(), vec![]),
            InputToken::text("Algolia search query: message, table".to_string(), vec![]),
        ],
        |text| text.encode_utf16().count(),
    );

    let ll_line = ll_line.run(&TextMatchAssignResolver::new_case_insensitive({
        [
            ("Slack".to_string(), vec![Service::Slack]),
            ("Algolia".to_string(), vec![Service::Algolia]),
            ("Magic".to_string(), vec![Service::Magic]),
            ("Wolfram".to_string(), vec![Service::Wolfram]),
        ]
        .iter()
        .cloned()
        .collect()
    }));

    let mut ll_display = LLLineDisplay::new(&ll_line);
    ll_display.include::<Service>();

    insta::assert_display_snapshot!(ll_display, @r###"
    when     Slack     hears     a     message     in     #  general  Algolia     search     query  :     message  ,     table
             ╰───╯Slack
                                                                      ╰─────╯Algolia
    "###);
}
