#![allow(dead_code)]

use insta;

use crate::ll_line::{
    ll, LLCursorAssignment, LLCursorStart, LLLine, LLLineDisplay, LLToken, Resolver, TextTag,
};

#[derive(Clone, Debug)]
enum CurrencySymbol {
    Euro,
    USDDollars,
    USDCents,
}

struct CurrencySymbolResolver;

impl Resolver for CurrencySymbolResolver {
    type Attr = CurrencySymbol;

    fn go(&self, start: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
        start
            .find_start_tag(&TextTag::SYMB)
            .into_iter()
            .filter_map(|(cur, sym_str)| {
                if let "$" = sym_str {
                    Some(cur.finish(CurrencySymbol::USDDollars))
                } else if let "€" = sym_str {
                    Some(cur.finish(CurrencySymbol::Euro))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Amount(f64);

struct AmountResolver {
    /// Configure for localization
    delimiters: Vec<char>,
    decimal: char,
}

impl Resolver for AmountResolver {
    type Attr = Amount;

    fn go(&self, cursor: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
        // 100.00
        // ---
        //     --
        cursor
            .find_start_tag(&TextTag::NATN)
            .into_iter()
            .map(|(cur, start_natn_str)| {
                dbg!("START");
                dbg!(start_natn_str);
                let mut number_string = String::from(start_natn_str);
                let mut last_valid_cursor = None;
                let mut current = cur;

                loop {
                    // skip delimiters going forwards
                    if let Some(delimeter_cur) =
                        current.match_forwards_char(self.delimiters.as_slice())
                    {
                        dbg!("delimeter");
                        last_valid_cursor = Some(current);
                        current = delimeter_cur;
                    } else {
                        break;
                    }

                    if let Some((following_delimeter_cur, natn_str)) =
                        current.match_forwards_tag(&TextTag::NATN)
                    {
                        number_string.push_str(natn_str);
                        last_valid_cursor = None;
                        current = following_delimeter_cur;
                    } else {
                        break;
                    }
                }

                // 100,,20
                if let Some(with_decimal_cur) = current.match_forwards_char(&[self.decimal]) {
                    dbg!("decimal");
                    last_valid_cursor = Some(current);
                    number_string.push('.');
                    current = with_decimal_cur;
                }

                loop {
                    if let Some((following_decimal_cur, natn_str)) =
                        current.match_forwards_tag(&TextTag::NATN)
                    {
                        number_string.push_str(natn_str);
                        last_valid_cursor = None;
                        current = following_decimal_cur;
                    } else {
                        break;
                    }

                    if let Some((trailing_delimeter_cur, natn_str)) =
                        current.match_forwards_tag(&TextTag::NATN)
                    {
                        number_string.push_str(natn_str);
                        last_valid_cursor = None;
                        current = trailing_delimeter_cur;
                    } else {
                        break;
                    }
                }

                last_valid_cursor
                    .unwrap_or(current)
                    .finish(Amount(number_string.parse::<f64>().unwrap()))
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct CurrencyAmount(CurrencySymbol, Amount);

struct CurrencyAmountResolver;

impl Resolver for CurrencyAmountResolver {
    type Attr = CurrencyAmount;

    fn go(&self, cursor: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
        cursor
            .find_start::<CurrencySymbol>()
            .into_iter()
            .filter_map(|(cur, curr_sym)| {
                // curr_sym
                cur.match_forwards::<Amount>()
                    .into_iter()
                    .next()
                    .or_else(|| cur.match_backwards::<Amount>().into_iter().next())
                    .map(|(cur_with_amt, amt)| {
                        cur_with_amt.finish(CurrencyAmount(curr_sym.clone(), amt.clone()))
                    })
            })
            .collect()
    }
}

fn ll_usd_1000_25() -> Vec<LLToken> {
    vec![
        ll(0, 1, TextTag::SYMB, "$"),
        ll(1, 2, TextTag::NATN, "1"),
        ll(2, 3, TextTag::PUNC, ","),
        ll(3, 6, TextTag::NATN, "000"),
        ll(6, 7, TextTag::PUNC, "."),
        ll(7, 9, TextTag::NATN, "25"),
    ]
}

fn ll_1000_25_euros() -> Vec<LLToken> {
    vec![
        ll(0, 0, TextTag::NATN, "1"),
        ll(0, 0, TextTag::SPACE, " "),
        ll(0, 0, TextTag::NATN, "000"),
        ll(0, 0, TextTag::PUNC, ","),
        ll(0, 0, TextTag::NATN, "25"),
        ll(0, 0, TextTag::SYMB, "€"),
    ]
}

#[test]
fn it_works() {
    let input = ll_usd_1000_25();
    let ll_line = LLLine::new(input)
        .run(&CurrencySymbolResolver)
        .run(&AmountResolver {
            delimiters: vec![','],
            decimal: '.',
        })
        .run(&CurrencyAmountResolver);

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<TextTag>();
    ll_line_display.include::<CurrencySymbol>();
    ll_line_display.include::<Amount>();
    ll_line_display.include::<CurrencyAmount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    $  1  ,  000  .  25
    ╰SYMB
       ╰NATN
          ╰PUNC
             ╰─╯NATN
                  ╰PUNC
                     ╰╯NATN
    ╰USDDollars
       ╰──────────────╯Amount(1000.25)
             ╰────────╯Amount(0.25)
                     ╰╯Amount(25.0)
    ╰─────────────────╯CurrencyAmount(USDDollars, Amount(1000.25))
    "###);
}
#[test]
fn it_works_euro() {
    let input = ll_1000_25_euros();
    let ll_line = LLLine::new(input)
        .run(&CurrencySymbolResolver)
        .run(&AmountResolver {
            delimiters: vec![' '],
            decimal: ',',
        })
        .run(&CurrencyAmountResolver);

    let mut ll_line_display = LLLineDisplay::new(&ll_line);
    ll_line_display.include::<TextTag>();
    ll_line_display.include::<CurrencySymbol>();
    ll_line_display.include::<Amount>();
    ll_line_display.include::<CurrencyAmount>();

    insta::assert_display_snapshot!(ll_line_display, @r###"
    1     000  ,  25  €
    ╰NATN
       ╰SPACE
          ╰─╯NATN
               ╰PUNC
                  ╰╯NATN
                      ╰─╯SYMB
                      ╰─╯Euro
    ╰──────────────╯Amount(1000.25)
          ╰────────╯Amount(0.25)
                  ╰╯Amount(25.0)
    ╰───────────────────╯CurrencyAmount(Euro, Amount(1000.25))
    "###);
}
