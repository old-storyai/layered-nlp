#![allow(dead_code)]

pub use p_tokens::{PToken, PTokenAt, PTokenRange, PTokenRangeTo};
use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
};
use type_map::TypeMap;

// impl NLPTokenSpan {
//     fn matches_text(&self, text: &str) -> bool {
//         self.tokens
//             .iter()
//             // TODO: split the same way that the tagger splits (using whitespace/punctuation)
//             .zip(text.split_whitespace())
//             .all(|(left, right)| match left.as_ref() {
//                 NLPToken::Text(val) => &val == &right,
//                 NLPToken::Atom => false,
//                 NLPToken::Expression => false,
//             })
//     }
// }

mod p_tokens;

struct PTokenLine {
    /// Indexed by [NLPTokenIndex]
    tokens: PTokenRange,
    /// A place to store and access overarching contextual information
    /// Perhaps, this should be readonly and required on creation...
    context_information: TypeMap,
    /// Collection of values that span over varying parts of the same
    annotations: BTreeMap<
        // start token index (inclusive)
        PTokenAt,
        BTreeMap< // bi-map?
            // end bound (exclusive)
            PTokenRangeTo,
            // attributes
            TypeMap,
        >,
    >,
}

pub enum AcceptResult<S, A> {
    Next(S, PTokenRangeTo),
    AssignToSpan(A),
    /// For when you want to assign attributes for the current context up until this end
    /// For example, a "Money recognizer" in `I paid them $5,000, and they thanked me.`
    /// will want to match `$5,000`–but not `$5,000,`, but at the moment we are building
    /// our state up to `$5,000,`, we don't know if that trailing `,` is a part of the number,
    /// or if it is a part of the number we're trying to parse.
    AssignUpTo(A, PTokenRangeTo),
}

struct StepContext<'a> {
    start: &'a PTokenAt,
    current: &'a PToken,
    attributes: &'a PTokenLine,
}

impl<'a> StepContext<'a> {
    fn current_has<T: 'static>(&self) -> Option<T> {
        unimplemented!()
    }
    fn current_within<T: 'static>(&self) -> Vec<(T, PTokenAt, PTokenRangeTo)> {
        unimplemented!()
    }
}

/// Roughly inspired by [fst Automaton](https://docs.rs/fst/0.4.5/src/fst/automaton/mod.rs.html#28-111)
trait Recognizer {
    /// The type of the state used in the automaton.
    type State;
    /// The attribute this automaton can assign
    type Attribute;

    /// Return the next state given `state` and an input.
    fn start(&self, next: &StepContext) -> Vec<AcceptResult<Self::State, Self::Attribute>>;

    /// Return the next state given `state` and a next context.
    fn next(
        &self,
        state: &Self::State,
        next: &StepContext,
    ) -> Vec<AcceptResult<Self::State, Self::Attribute>>;
}

impl PTokenLine {
    fn apply<R, S>(&mut self, _rule: R)
    where
        R: Recognizer<State = S>,
    {
        todo!("actually try to execute the rule over the span")
    }
}

struct RecognizedLocation {
    query: String,
    country: Option<String>,
}

mod numbers {
    use super::*;

    // TODO: configure for alternative language number punctuations (like french numbers `$1.000.000,00`)
    #[derive(Default)]
    struct NumberRecognizer(());

    struct RecognizedNumber(rust_decimal::Decimal);
    enum State {
        /// in "123" "," "4"
        /// .0 "1234"
        BeforeDecimal(String),
        /// in "123" "," "4" "." "5"
        /// .0 "1234"
        /// .1 "5"
        AfterDecimal(String, String),
    }

    impl Recognizer for NumberRecognizer {
        type State = State;

        type Attribute = RecognizedNumber;

        fn start(&self, next: &StepContext) -> Vec<AcceptResult<Self::State, Self::Attribute>> {
            
            
        }

        fn next(
            &self,
            state: &Self::State,
            next: &StepContext,
        ) -> Vec<AcceptResult<Self::State, Self::Attribute>> {
            todo!()
        }
    }
}

mod money {
    use super::*;

    enum CurrencySymbol {
        USD,
    }
    enum LastPunctuation {
        DecimalPoint,
        NumberVisualSeparator,
    }
    enum State {
        SymbolStart(CurrencySymbol),
        SymbolWithNumbersLeftOfDecimal {
            currency: CurrencySymbol,
            current_left_of_decimal: usize,
            parsed_up_to: PTokenRangeTo,
            last_punctuation: Option<LastPunctuation>,
        },
        // SymbolWithNumbersRightOfDecimal {
    }

    enum MoneyAmount {
        USDPennies(usize),
    }
    pub struct RecognizedMoney {
        /// e.g. "$5.00" or "$50k"
        query: String,
        amount: MoneyAmount,
    }

    // TODO: configure for alternative language number punctuations (like french numbers `$1.000.000,00`)
    #[derive(Default)]
    struct MoneyRecognizer(());

    impl Recognizer for MoneyRecognizer {
        type State = State;

        type Attribute = RecognizedMoney;

        fn start(&self, next: &StepContext) -> Vec<AcceptResult<Self::State, Self::Attribute>> {
            
        }

        fn next(
            &self,
            state: &Self::State,
            next: &StepContext,
        ) -> Vec<AcceptResult<Self::State, Self::Attribute>> {
            todo!()
        }
    }
}

#[allow(dead_code)]
fn t() {
    /*
    let mut tree = NLPTokenGraph {
        starting_edges: set1(ToEdge::To(0)),
        edges: vec3(
            (
                NLPToken {
                    attributes: Default::default(),
                    from_position: 0,
                    to_position: 3,
                    tokens: vec1(Arc::new(text("new"))),
                },
                set1(ToEdge::To(1)),
            ),
            (
                NLPToken {
                    attributes: Default::default(),
                    from_position: 4,
                    to_position: 8,
                    tokens: vec1(Arc::new(text("york"))),
                },
                set1(ToEdge::To(2)),
            ),
            (
                NLPToken {
                    attributes: Default::default(),
                    from_position: 9,
                    to_position: 13,
                    tokens: vec1(Arc::new(text("city"))),
                },
                set1(ToEdge::End),
            ),
        ),
    };

    tree.apply((), |state, edge| {
        (
            Some(state.clone()),
            if edge.matches_text("Amsterdam") {
                vec1(GraphAction::assign(RecognizedLocation {
                    country: Some(s("Netherlands")),
                    query: s("Amsterdam, Netherlands"),
                }))
            } else {
                vec0()
            },
        )
    })

    // //
    // for tree in branches
    //     .iter()
    //     .retain(|tree| tree.matches::<SentimentAttribute>(|sentiment| sentiment.positive > 5))
    // {
    //     tree.attrs.setAttribute(Positive);
    // }
    */
}

fn set1<T: Eq + Hash>(z: T) -> HashSet<T> {
    std::iter::once(z).collect()
}
fn set2<T: Eq + Hash>(y: T, z: T) -> HashSet<T> {
    vec![y, z].into_iter().collect()
}
fn set3<T: Eq + Hash>(x: T, y: T, z: T) -> HashSet<T> {
    vec![x, y, z].into_iter().collect()
}
