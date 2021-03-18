#![allow(dead_code, unused_variables)]
trait IntoMatchItem {
    type MatchItem: for<'a> MatchItem<'a>;
}

trait MatchItem<'a> {
    type Out;
}

// type Phantom<'a> = std::marker::PhantomData<&'a mut ()>;

// what "defines" behavior (equivalent of *Borrower in shipyard)
/// Never constructed, just used to get a GAT-like behavior.
struct ZeroOrMorePattern<T: for<'a> MatchItem<'a>>(T);
// what you pull out o the pattern's result
struct ZeroOrMore<TMatchInto: IntoMatchItem>(TMatchInto);

impl<TMatchInto: IntoMatchItem> IntoMatchItem for ZeroOrMore<TMatchInto> {
    type MatchItem = ZeroOrMorePattern<TMatchInto::MatchItem>;
}

impl<'a, A: for<'b> MatchItem<'b>> MatchItem<'a> for ZeroOrMorePattern<A> {
    type Out = Option<<A as MatchItem<'a>>::Out>;
}

struct Matches1<A: IntoMatchItem>(A);
struct Matches2<A: IntoMatchItem, B: IntoMatchItem>(A, B);
struct Matches3<A: IntoMatchItem, B: IntoMatchItem, C: IntoMatchItem>(A, B, C);

impl<A: IntoMatchItem, B: IntoMatchItem> Matches2<A, B> {
    fn then<C: IntoMatchItem>(self, next: C) -> Matches3<A, B, C> {
        let Matches2(a, b) = self;
        Matches3(a, b, next)
    }
    fn zero_or_more<C: IntoMatchItem>(self, next: C) -> Matches3<A, B, ZeroOrMore<C>> {
        let Matches2(a, b) = self;
        Matches3(a, b, ZeroOrMore(next))
    }
}
