use layered_nlp::*;

fn main() {
    let ll_line = create_tokens::create_tokens(vec![], |_| 0);

    insta::assert_debug_snapshot!(ll_line.ll_tokens(), @"[]");
}
