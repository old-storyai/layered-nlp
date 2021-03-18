use crate::ll_line::{x, LLSelection};

use super::*;

#[derive(Clone, Debug)]
enum CurrencySymbol {
    Euro,
    USDDollars,
}

struct CurrencySymbolResolver;

impl Resolver for CurrencySymbolResolver {
    type Attr = CurrencySymbol;

    fn go(&self, start: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        start
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

        while let Some((mut sel, (_, text))) = search_range_sel
            .find_first_by(&(x::attr_eq(&TextTag::NATN), x::token_text()))
            .into_iter()
            .next()
        {
            let mut number_string = String::from(text);
            let mut last_valid_cursor = None;

            loop {
                if let Some((delimeter_sel, _)) =
                    sel.match_first_forwards(&x::token_has_any(self.delimiters.as_slice()))
                {
                    last_valid_cursor = Some(sel);
                    sel = delimeter_sel;
                } else {
                    break;
                }

                if let Some((following_delimeter_cur, (_, text))) =
                    sel.match_first_forwards(&(x::attr_eq(&TextTag::NATN), x::token_text()))
                {
                    number_string.push_str(text);
                    last_valid_cursor = None;
                    sel = following_delimeter_cur;
                } else {
                    break;
                }
            }

            // 100,,20
            if let Some((with_decimal_sel, _)) =
                sel.match_first_forwards(&x::token_has_any(&[self.decimal]))
            {
                last_valid_cursor = Some(sel);
                number_string.push('.');
                sel = with_decimal_sel;
            }

            if let Some((following_decimal_cur, ((), text))) =
                sel.match_first_forwards(&(x::attr_eq(&TextTag::NATN), x::token_text()))
            {
                number_string.push_str(text);
                last_valid_cursor = None;
                sel = following_decimal_cur;
            }

            attrs.push(
                last_valid_cursor
                    .unwrap_or(sel.clone())
                    .finish_with_attr(Amount(number_string.parse::<Decimal>().unwrap())),
            );

            if let [_, Some(right_sel)] = search_range_sel.split_with(&sel) {
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

    fn go(&self, cursor: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        cursor
            .find_by(&x::attr::<CurrencySymbol>())
            .into_iter()
            .filter_map(|(sel, curr_sym)| {
                sel.match_first_forwards(&x::attr::<Amount>())
                    .or_else(|| sel.match_first_backwards(&x::attr::<Amount>()))
                    .map(|(cur_with_amt, amt)| {
                        cur_with_amt.finish_with_attr(CurrencyAmount(curr_sym.clone(), amt.clone()))
                    })
            })
            .collect()
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
