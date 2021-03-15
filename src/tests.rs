use insta;
use rust_decimal::Decimal;

use crate::create_tokens::*;
use crate::ll_line::{LLCursorAssignment, LLCursorStart, LLLine, LLLineDisplay, Resolver, TextTag};

fn split_input(input: &str) -> String {
    format!("{:#?}", test_line(input).ll_tokens())
}

#[derive(Clone, Debug)]
enum CurrencySymbol {
    Euro,
    USDDollars,
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
struct Amount(Decimal);

struct AmountResolver {
    /// Configure for localization
    delimiters: Vec<char>,
    decimal: char,
}

impl Resolver for AmountResolver {
    type Attr = Amount;

    fn go(&self, mut start: LLCursorStart) -> Vec<LLCursorAssignment<Self::Attr>> {
        let mut attrs = Vec::new();

        while let Some((cur, start_natn_str)) = start.find_next_start_tag(&TextTag::NATN) {
            let mut number_string = String::from(start_natn_str);
            let mut last_valid_cursor = None;
            let mut current = cur;

            loop {
                // skip delimiters going forwards
                if let Some(delimeter_cur) = current.match_forwards_char(self.delimiters.as_slice())
                {
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

            // ensure that we perform our next iteration after the matched so we
            // don't accidentally include the natns in the already finished Amount
            start = current.start_after();

            attrs.push(
                last_valid_cursor
                    .unwrap_or(current)
                    .finish(Amount(number_string.parse::<Decimal>().unwrap())),
            );
        }

        attrs
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

fn ll_usd_1000_25() -> LLLine {
    test_line("$1,000.25")
}

fn ll_1000_25_euros() -> LLLine {
    test_line(". 1 000,25€")
}

#[test]
fn it_works() {
    let ll_line = ll_usd_1000_25()
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
    let ll_line = ll_1000_25_euros()
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

#[cfg(test)]
pub(crate) fn test_line(input: &str) -> LLLine {
    create_tokens(
        vec![InputToken::Text {
            text: input.to_string(),
            attrs: Vec::new(),
        }],
        |text| text.encode_utf16().count(),
    )
}

#[test]
fn test_tokenizing() {
    let input = ". 1 000.23. € .5";

    insta::assert_display_snapshot!(split_input(&input), @r###"
    [
        LLToken {
            token_idx: 0,
            pos_starts_at: 0,
            pos_ends_at: 1,
            token: Text(
                ".",
                PUNC,
            ),
        },
        LLToken {
            token_idx: 1,
            pos_starts_at: 1,
            pos_ends_at: 2,
            token: Text(
                " ",
                SPACE,
            ),
        },
        LLToken {
            token_idx: 2,
            pos_starts_at: 2,
            pos_ends_at: 3,
            token: Text(
                "1",
                NATN,
            ),
        },
        LLToken {
            token_idx: 3,
            pos_starts_at: 3,
            pos_ends_at: 4,
            token: Text(
                " ",
                SPACE,
            ),
        },
        LLToken {
            token_idx: 4,
            pos_starts_at: 4,
            pos_ends_at: 7,
            token: Text(
                "000",
                NATN,
            ),
        },
        LLToken {
            token_idx: 5,
            pos_starts_at: 7,
            pos_ends_at: 8,
            token: Text(
                ".",
                PUNC,
            ),
        },
        LLToken {
            token_idx: 6,
            pos_starts_at: 8,
            pos_ends_at: 10,
            token: Text(
                "23",
                NATN,
            ),
        },
        LLToken {
            token_idx: 7,
            pos_starts_at: 10,
            pos_ends_at: 11,
            token: Text(
                ".",
                PUNC,
            ),
        },
        LLToken {
            token_idx: 8,
            pos_starts_at: 11,
            pos_ends_at: 12,
            token: Text(
                " ",
                SPACE,
            ),
        },
        LLToken {
            token_idx: 9,
            pos_starts_at: 12,
            pos_ends_at: 13,
            token: Text(
                "€",
                SYMB,
            ),
        },
        LLToken {
            token_idx: 10,
            pos_starts_at: 13,
            pos_ends_at: 14,
            token: Text(
                " ",
                SPACE,
            ),
        },
        LLToken {
            token_idx: 11,
            pos_starts_at: 14,
            pos_ends_at: 15,
            token: Text(
                ".",
                PUNC,
            ),
        },
        LLToken {
            token_idx: 12,
            pos_starts_at: 15,
            pos_ends_at: 16,
            token: Text(
                "5",
                NATN,
            ),
        },
    ]
    "###);
}
