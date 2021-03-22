use crate::ll_line::{x, LLSelection};

use super::*;

#[derive(Clone, Debug)]
enum CurrencySymbol {
    Euro,
    USDDollars,
    Yen,
}

struct CurrencySymbolResolver;

impl Resolver for CurrencySymbolResolver {
    type Attr = CurrencySymbol;

    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        selection
            .find_by(&x::token_has_any(&['€', '$', '¥']))
            .into_iter()
            .map(|(sel, sym_ch)| match sym_ch {
                '$' => sel.finish_with_attr(CurrencySymbol::USDDollars),
                '€' => sel.finish_with_attr(CurrencySymbol::Euro),
                '¥' => sel.finish_with_attr(CurrencySymbol::Yen),
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

    fn go(&self, mut selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        let mut attrs = vec![];

        while let Some((mut sel, (_, text))) =
            selection.find_first_by(&(x::attr_eq(&TextTag::NATN), x::token_text()))
        {
            let mut number_string = String::from(text);

            // Could we have an enum here perhaps something like:
            // enum {
            //   NaturalNumber(?),
            //   TrailingDelimiter(?),
            //   TrailingDecimal(?)
            // }
            let mut last_valid_cursor = None;

            // Avoid trailing delimeters
            loop {
                if let Some((delimeter_sel, _)) =
                    // extend_forwards?
                // extend_if_hugging_forwards?
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

            if last_valid_cursor.is_none() {
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
            }

            // dbg!(number_string.clone());

            attrs.push(
                last_valid_cursor
                    .unwrap_or_else(|| sel.clone())
                    .finish_with_attr(Amount(dbg!(number_string).parse::<Decimal>().unwrap())),
            );

            if let [_, Some(right_sel)] = selection.split_with(&sel) {
                selection = right_sel;
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
fn amount_resolver_foo() {
    let ll_line = test_line("100,.000").run(&AmountResolver {
        delimiters: vec![','],
        decimal: '.',
    });

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<Amount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    100  ,  .  000
    ╰─╯Amount(100)
               ╰─╯Amount(0)
    "###);
}

#[test]
fn amount_resolver_bar() {
    let ll_line = test_line("10.00,00").run(&AmountResolver {
        delimiters: vec![','],
        decimal: '.',
    });

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<Amount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    10  .  00  ,  00
    ╰───────╯Amount(10.00)
                  ╰╯Amount(0)
    "###);
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
fn it_works_yen() {
    let ll_line = test_line("¥227")
        .run(&CurrencySymbolResolver)
        .run(&AmountResolver {
            delimiters: vec![','],
            // TODO: What if a currency does not have decimals?
            decimal: ' ',
        })
        .run(&CurrencyAmountResolver);

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<CurrencySymbol>();
    ll_line_display.include::<Amount>();
    ll_line_display.include::<CurrencyAmount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    ¥  227
    ╰Yen
       ╰─╯Amount(227)
    ╰────╯CurrencyAmount(Yen, Amount(227))
    "###);
}

#[derive(Clone, Debug)]
enum WillCaresSymbol {
    Tests,
}

struct WillCaresResolver;

impl Resolver for WillCaresResolver {
    type Attr = WillCaresSymbol;

    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        selection
            .find_by(&(x::attr_eq(&TextTag::WORD), x::token_text()))
            // .find_by(&x::token_has_any(&["hens"]))
            .into_iter()
            .map(|(sel, (_, sym_ch))| match sym_ch {
                "tests" => sel.finish_with_attr(WillCaresSymbol::Tests),
                _ => unreachable!(),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct WillCaresAmount(WillCaresSymbol, Amount);

struct WillCaresAmountResolver;

impl Resolver for WillCaresAmountResolver {
    type Attr = WillCaresAmount;

    fn go(&self, selection: LLSelection) -> Vec<LLCursorAssignment<Self::Attr>> {
        // find_by(Symbol AND Amount OR Amount AND Symbol)
        selection
            .find_by(&x::attr::<WillCaresSymbol>())
            .into_iter()
            .filter_map(|(sel, will_cares_sym)| {
                // How about sel.match_first(Prefer::Forwards)?
                sel.match_first(crate::ll_line::Direction::Forwards, &x::attr::<Amount>())
                    .map(|(will_cares_with_amt, amt)| {
                        will_cares_with_amt
                            .finish_with_attr(WillCaresAmount(will_cares_sym.clone(), amt.clone()))
                    })
            })
            .collect()
    }
}

#[test]
fn it_works_for_things_that_will_cares_about() {
    let ll_line = test_line("1337tests")
        .run(&WillCaresResolver)
        .run(&AmountResolver {
            delimiters: vec![','],
            // TODO: What if a currency does not have decimals?
            decimal: ' ',
        })
        .run(&WillCaresAmountResolver);

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<WillCaresSymbol>();
    ll_line_display.include::<Amount>();
    ll_line_display.include::<WillCaresAmount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    1337  tests
          ╰───╯Tests
    ╰──╯Amount(1337)
    ╰─────────╯WillCaresAmount(Tests, Amount(1337))
    "###);
}
