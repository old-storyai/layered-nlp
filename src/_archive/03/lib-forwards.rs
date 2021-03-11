use std::rc::Rc;

mod nlp_token_graph;

struct AttributesMap {}

struct PToken<T> {
    token: Rc<T>,
    map: Rc<AttributesMap>,
    // next: Option<Rc<PToken<T>>>,
    // previous: Option<Rc<PToken<T>>>,
}

/// Pinned to some token
struct AnnotationBuilder<T, A> {
    _t: T,
    _a: A,
}

enum SearchResult<A> {
    Skip,
    Next,
    Finish(A),
}

impl<T, A> AnnotationBuilder<T, A> {
    fn search_forwards<F>(&mut self, each: F)
    where
        F: (FnMut(PToken<T>) -> SearchResult<A>),
    {
        unimplemented!()
    }
}

trait Annotator<T> {
    type Annotation;
    fn start<'t>(pt: &PToken<T>);
}

struct NumberPlugin {}

struct NumberMatcher {}

// trait Matcher {
//     fn search_forwards(&self, )
// }
