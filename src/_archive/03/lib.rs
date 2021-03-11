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
    Continue,
    Finish(A),
}

impl<T, A> AnnotationBuilder<T, A> {
    fn next_token_is<F>(&mut self, test: F)
    where
        F: (Fn(PToken<T>) -> SearchResult<A>),
    {
        unimplemented!()
    }
}

trait Annotator<T> {
    type Annotation;
    fn start<'t>(
        pt: &PToken<T>,
    ) -> Option<Box<dyn FnOnce(&mut AnnotationBuilder<T, Self::Annotation>)>>;
}

struct NumberPlugin {}

struct RecognizedNumber {}

impl Annotator<String> for NumberPlugin {
    type Annotation = RecognizedNumber;

    fn start<'t>(
        builder: &mut AnnotationBuilder<String, Self::Annotation>
    ) {
        builder.next_token_is(|p| {
            SearchResult::Next
        })
    }
}

trait AndThenBool {
    fn true_and_then<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>;
}

impl AndThenBool for bool {
    fn true_and_then<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        if self {
            f()
        } else {
            None
        }
    }
}
// trait Matcher {
//     fn search_forwards(&self, )
// }
