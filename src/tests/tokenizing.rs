use super::*;

fn split_input(input: &str) -> String {
    format!("{:#?}", test_line(input).ll_tokens())
}

#[test]
fn test_tokenizing() {
    let input = ". 1 000.23. € .5";

    insta::assert_display_snapshot!(split_input(input), @r###"
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
