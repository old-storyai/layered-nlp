use crate::ll_line::{x, FinishWith, LLSelection};

use super::*;

#[derive(Clone, Debug)]
enum CurrencySymbol {
    Euro,
    USDDollars,
}

struct CurrencySymbolResolver;

impl Resolver for CurrencySymbolResolver {
    type Attr = CurrencySymbol;

    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        selection
            .find_by(&x::token_has_any(&['€', '$']))
            .into_iter()
            .map(|(sel, sym_ch)| match sym_ch {
                '$' => sel.finish_with_attr(CurrencySymbol::USDDollars),
                '€' => sel.finish_with_attr(CurrencySymbol::Euro),
                _ => unreachable!(),
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Amount(Decimal);

struct AmountResolver {
    /// Configure for localization
    delimiters: Vec<char>,
    decimal: char,
}

impl Resolver for AmountResolver {
    type Attr = Amount;

    fn go(&self, mut search_range_sel: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        let mut attrs = vec![];

        while let Some((mut selection, (_, text))) = search_range_sel
            .find_first_by(&x::any_of((x::attr_eq(&TextTag::NATN), x::token_text())))
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
                    .match_first_forwards(&x::any_of((x::attr_eq(&TextTag::NATN), x::token_text())))
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
                    .match_first_forwards(&x::any_of((x::attr_eq(&TextTag::NATN), x::token_text())))
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

#[derive(Debug, Clone)]
struct CurrencyAmount(CurrencySymbol, Amount);

struct CurrencyAmountResolver;

impl Resolver for CurrencyAmountResolver {
    type Attr = CurrencyAmount;

    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        selection
            .find_by_forwards_and_backwards(&x::seq((
                x::attr::<CurrencySymbol>(),
                x::attr::<Amount>(),
            )))
            .finish_with(|(cur_sym, amt)| CurrencyAmount(cur_sym.clone(), amt.clone()))
    }
}

#[test]
fn it_works_usd() {
    let ll_line = test_line("$1,000.25")
        .run(&CurrencySymbolResolver)
        .run(&AmountResolver {
            delimiters: vec![','],
            decimal: '.',
        })
        .run(&CurrencyAmountResolver);

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<CurrencySymbol>();
    ll_line_display.include::<Amount>();
    ll_line_display.include::<CurrencyAmount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    $  1  ,  000  .  25
    ╰USDDollars
       ╰──────────────╯Amount(1000.25)
    ╰─────────────────╯CurrencyAmount(USDDollars, Amount(1000.25))
    "###);
}
#[test]
fn it_works_euro() {
    let ll_line = test_line(". 1 000,25€")
        .run(&CurrencySymbolResolver)
        .run(&AmountResolver {
            delimiters: vec![' '],
            decimal: ',',
        })
        .run(&CurrencyAmountResolver);

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<CurrencySymbol>();
    ll_line_display.include::<Amount>();
    ll_line_display.include::<CurrencyAmount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    .     1     000  ,  25  €
                            ╰Euro
          ╰──────────────╯Amount(1000.25)
          ╰─────────────────╯CurrencyAmount(Euro, Amount(1000.25))
    "###);
}

#[test]
fn trailing_delimiter() {
    let ll_line = test_line("100,.").run(&AmountResolver {
        delimiters: vec![','],
        decimal: '.',
    });

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<Amount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    100  ,  .
    ╰─╯Amount(100)
    "###);
}
