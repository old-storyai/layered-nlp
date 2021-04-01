use super::x::{XBackwards, XForwards};
use super::{assert_ll_lines_equals, LLCursorAssignment, LLLine, Rc, XMatch};

// Cutting Selection (selection) -> Iter<selection>
//  - split_by_x             :: [aaaxaaaxaaa] -> [aaa]x[aaa]x[aaa]
//  - find_by_x              :: [aaaxaaaxaaa] -> aaa[x]aaa[x]aaa
// skips overlapping
//  - split_by_forwards_x    :: [aaaxaaaxaaa] -> [aaa]x[aaa]x[aaa]
//  - find_by_forwards_x     :: [aaaxaaaxaaa] -> aaa[x]aaa[x]aaa
// skips overlapping
//  - split_by_backwards_x   :: [aaaxaaaxaaa] -> [aaa]x[aaa]x[aaa]
//  - find_by_backwards_x    :: [aaaxaaaxaaa] -> aaa[x]aaa[x]aaa
// Expand Selection (selection) -> Iter<selection>
//  - match_forwards_x          :: x[aaa]x -> x[aaax]
//  - match_backwards_x         :: x[aaa]x -> [xaaa]x
// Shrinking Selection (selection) -> Option<selection>
//  - trim_x                    :: [xaaax] -> x[aaa]x
//  - trim_leading_x            :: [xaaax] -> x[aaax]
//  - trim_trailing_x           :: [xaaax] -> [xaaa]x

// x
// start large, then test

/// Selections will never be empty
#[derive(Clone)]
pub struct LLSelection {
    pub(super) ll_line: Rc<LLLine>,
    /// Where to begin in the line (inclusive, default is 0)
    pub(super) start_idx: usize,
    /// Where to end in the line (inclusive, default is last idx)
    pub(super) end_idx: usize,
}

impl std::fmt::Debug for LLSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LLSelection")
            .field("start_idx", &self.start_idx)
            .field("end_idx", &self.end_idx)
            .finish()
    }
}

impl PartialEq for LLSelection {
    fn eq(&self, other: &Self) -> bool {
        self.start_idx == other.start_idx
            && self.end_idx == other.end_idx
            && Rc::ptr_eq(&self.ll_line, &other.ll_line)
    }
}

impl LLSelection {
    /// Returns None if the line is empty
    pub fn from_line(ll_line: Rc<LLLine>) -> Option<Self> {
        let line_len = ll_line.ll_tokens.len();
        if line_len > 0 {
            Some(LLSelection {
                ll_line,
                end_idx: line_len - 1,
                start_idx: 0,
            })
        } else {
            None // empty selection
        }
    }

    pub fn split_by<'a, M: XMatch<'a>>(&'a self, matcher: &M) -> Vec<LLSelection> {
        let matches = self.find_by(matcher);

        if matches.is_empty() {
            return vec![self.clone()];
        }

        if matches.len() == 1 {
            return matches[0]
                .0
                .start_idx
                .checked_sub(1)
                .and_then(|end_idx| self.selection_from(self.start_idx, end_idx))
                .into_iter()
                .chain(self.selection_from(matches[0].0.end_idx + 1, self.end_idx))
                .collect();
        }

        let start_opt = matches[0]
            .0
            .start_idx
            .checked_sub(1)
            .and_then(|end_idx| self.selection_from(self.start_idx, end_idx));

        let end_opt = self.selection_from(matches.last().unwrap().0.end_idx + 1, self.end_idx);
        start_opt
            .into_iter()
            .chain(matches.windows(2).into_iter().filter_map(|m| {
                m[1].0
                    .start_idx
                    .checked_sub(1)
                    .and_then(|end_idx| self.selection_from(m[0].0.end_idx + 1, end_idx))
            }))
            .chain(end_opt)
            .collect()
    }

    pub fn find_by<'a, M: XMatch<'a>>(&'a self, matcher: &M) -> Vec<(LLSelection, M::Out)> {
        (self.start_idx..=self.end_idx)
            .map(|i| {
                let forwards = XForwards { from_idx: i };

                matcher
                    .go(&forwards, &self.ll_line)
                    .into_iter()
                    .map(move |(out, next_idx)| {
                        (
                            LLSelection {
                                start_idx: i,
                                end_idx: next_idx.0,
                                ll_line: self.ll_line.clone(),
                            },
                            out,
                        )
                    })
            })
            .flatten()
            .collect()
    }

    pub fn find_first_by<'a, M: XMatch<'a>>(
        &'a self,
        matcher: &M,
    ) -> Option<(LLSelection, M::Out)> {
        self.find_by(matcher).into_iter().next()
    }

    pub fn find_by_forwards_and_backwards<'a, M: XMatch<'a>>(
        &'a self,
        matcher: &M,
    ) -> Vec<(LLSelection, M::Out)> {
        (self.start_idx..=self.end_idx)
            .map(|i| {
                let forwards = XForwards { from_idx: i };

                matcher
                    .go(&forwards, &self.ll_line)
                    .into_iter()
                    .map(move |(out, next_idx)| {
                        (
                            LLSelection {
                                start_idx: i,
                                end_idx: next_idx.0,
                                ll_line: self.ll_line.clone(),
                            },
                            out,
                        )
                    })
                    .chain({
                        let backwards = XBackwards { from_idx: i };

                        matcher.go(&backwards, &self.ll_line).into_iter().map(
                            move |(out, next_idx)| {
                                (
                                    LLSelection {
                                        start_idx: next_idx.0,
                                        end_idx: i,
                                        ll_line: self.ll_line.clone(),
                                    },
                                    out,
                                )
                            },
                        )
                    })
            })
            .flatten()
            .collect()
    }

    pub fn match_forwards<'a, M: XMatch<'a>>(&'a self, matcher: &M) -> Vec<(LLSelection, M::Out)> {
        // [ ... ] - Current selection
        //        [ ... ] - Trying to match Attr
        if self.end_idx + 1 == self.ll_line.ll_tokens.len() {
            return Vec::new();
        }

        let forwards = XForwards {
            from_idx: self.end_idx + 1,
        };

        matcher
            .go(&forwards, &self.ll_line)
            .into_iter()
            .map(|(out, next_idx)| {
                (
                    LLSelection {
                        start_idx: self.start_idx,
                        end_idx: next_idx.0,
                        ll_line: self.ll_line.clone(),
                    },
                    out,
                )
            })
            .collect()
    }

    pub fn match_first_forwards<'a, M: XMatch<'a>>(
        &'a self,
        matcher: &M,
    ) -> Option<(LLSelection, M::Out)> {
        self.match_forwards(matcher).into_iter().next()
    }

    pub fn match_forwards_longest<'a, M: XMatch<'a>>(
        &'a self,
        _matcher: &M,
    ) -> Option<(LLSelection, M::Out)> {
        todo!()
    }

    pub fn match_forwards_shortest<'a, M: XMatch<'a>>(
        &'a self,
        _matcher: &M,
    ) -> Option<(LLSelection, M::Out)> {
        todo!()
    }

    pub fn match_backwards<'a, M: XMatch<'a>>(&'a self, matcher: &M) -> Vec<(LLSelection, M::Out)> {
        if self.start_idx == 0 {
            return Vec::new();
        }

        let backwards = XBackwards {
            from_idx: self.start_idx - 1,
        };

        matcher
            .go(&backwards, &self.ll_line)
            .into_iter()
            .map(|(out, next_idx)| {
                (
                    LLSelection {
                        start_idx: next_idx.0,
                        end_idx: self.end_idx,
                        ll_line: self.ll_line.clone(),
                    },
                    out,
                )
            })
            .collect()
    }

    pub fn match_first_backwards<'a, M: XMatch<'a>>(
        &'a self,
        matcher: &M,
    ) -> Option<(LLSelection, M::Out)> {
        self.match_backwards(matcher).into_iter().next()
    }

    pub fn after(&self) -> Option<LLSelection> {
        let ll_line_end = self.ll_line.ll_tokens.len() - 1;

        if self.start_idx + 1 == ll_line_end {
            None
        } else {
            Some(LLSelection {
                ll_line: self.ll_line.clone(),
                start_idx: self.end_idx + 1,
                end_idx: ll_line_end,
            })
        }
    }

    fn selection_from(&self, mut start_idx: usize, mut end_idx: usize) -> Option<LLSelection> {
        start_idx = start_idx.max(self.start_idx);
        end_idx = end_idx.min(self.end_idx);
        if start_idx <= end_idx {
            Some(LLSelection {
                ll_line: self.ll_line.clone(),
                end_idx,
                start_idx,
            })
        } else {
            None
        }
    }

    // Hmmm... TODO: Unit test this thoroughly
    pub fn split_with(&self, other_selection: &LLSelection) -> [Option<LLSelection>; 2] {
        assert_ll_lines_equals(&self.ll_line, &other_selection.ll_line);

        // 1   [          ]
        // 2                   [      ]
        //     [          ]
        //               ]      [
        // 1   [          ]
        // 2           [      ]
        //     [      ]

        [
            if other_selection.start_idx > 0 {
                // underflow protection
                self.selection_from(self.start_idx, other_selection.start_idx - 1)
            } else {
                None
            },
            self.selection_from(other_selection.end_idx + 1, self.end_idx),
        ]
    }

    pub fn finish_with_attr<Attr>(&self, value: Attr) -> LLCursorAssignment<Attr> {
        LLCursorAssignment {
            end_idx: self.end_idx,
            start_idx: self.start_idx,
            value,
        }
    }
}
