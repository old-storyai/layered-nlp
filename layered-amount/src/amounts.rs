use layered_nlp::{x, LLCursorAssignment, LLSelection, Resolver, TextTag};
use rust_decimal::Decimal;

#[derive(Clone, Debug)]
pub struct Amount(Decimal);

impl Amount {
    pub fn get_decimal(&self) -> &Decimal {
        &self.0
    }
    pub fn new<T: Into<Decimal>>(from: T) -> Self {
        Self(from.into())
    }
}

impl From<Decimal> for Amount {
    fn from(dec: Decimal) -> Self {
        Amount(dec)
    }
}

pub struct AmountResolver {
    /// Configure for localization
    delimiters: Vec<char>,
    decimal: char,
}

impl AmountResolver {
    pub fn new(delimiters: Vec<char>, decimal: char) -> Self {
        Self {
            delimiters,
            decimal,
        }
    }
    /// Default comma `,` and apostrophe `'` delimeters, with period `.` decimal point.
    ///  * `1'000'240.00` = `1000240.00`
    ///  * `1,000,000` = `1000000`
    ///  * `1,00,00` = `10000`
    ///  * `1.00,00` = `1.0000`
    pub fn english() -> Self {
        Self {
            delimiters: vec![',', '\''],
            decimal: '.',
        }
    }
    /// Default period `.`, apostrophe `'`, and space ` ` delimeters, with comma `,` decimal point.
    ///  * `1'000'240,00` = `1000240.00`
    ///  * `1 000 000` = `1000000`
    ///  * `1,00` = `1.00`
    ///  * `1.00,00` = `100.00`
    pub fn french() -> Self {
        Self {
            delimiters: vec![' ', '.', '\''],
            decimal: '.',
        }
    }
}

impl Resolver for AmountResolver {
    type Attr = Amount;

    fn go(&self, mut search_range_sel: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        let mut attrs = vec![];

        while let Some((mut selection, (_, text))) =
            search_range_sel.find_first_by(&x::all((x::attr_eq(&TextTag::NATN), x::token_text())))
        {
            let mut number_string = String::from(text);
            let mut last_valid_selection = None;

            // Avoid trailing delimeters
            loop {
                if let Some((delimeter_sel, _)) =
                    selection.match_first_forwards(&x::token_has_any(self.delimiters.as_slice()))
                {
                    last_valid_selection = Some(selection);
                    selection = delimeter_sel;
                } else {
                    break;
                }

                if let Some((following_delimeter_sel, (_, text))) = selection
                    .match_first_forwards(&x::all((x::attr_eq(&TextTag::NATN), x::token_text())))
                {
                    number_string.push_str(text);
                    last_valid_selection = None;
                    selection = following_delimeter_sel;
                } else {
                    break;
                }
            }

            if last_valid_selection.is_none() {
                // 100,,20
                if let Some((with_decimal_sel, _)) =
                    selection.match_first_forwards(&x::token_has_any(&[self.decimal]))
                {
                    last_valid_selection = Some(selection);
                    number_string.push('.');
                    selection = with_decimal_sel;
                }

                if let Some((following_decimal_sel, ((), text))) = selection
                    .match_first_forwards(&x::all((x::attr_eq(&TextTag::NATN), x::token_text())))
                {
                    number_string.push_str(text);
                    last_valid_selection = None;
                    selection = following_decimal_sel;
                }
            }

            attrs.push(
                last_valid_selection
                    .unwrap_or_else(|| selection.clone())
                    .finish_with_attr(Amount(number_string.parse::<Decimal>().unwrap())),
            );

            if let [_, Some(right_sel)] = search_range_sel.split_with(&selection) {
                search_range_sel = right_sel;
            } else {
                break;
            }
        }

        attrs
    }
}
