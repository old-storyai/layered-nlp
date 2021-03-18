
#[derive(Clone)]
pub struct LLCursor {
    // private
    start_idx: usize,
    end_idx: usize,
    ll_line: Rc<LLLine>,
}

impl LLCursor {
    pub fn expand_forwards(&self, to_end_idx: usize) -> Self {
        assert!(self.end_idx <= to_end_idx);
        LLCursor {
            start_idx: self.start_idx,
            end_idx: to_end_idx,
            ll_line: self.ll_line.clone(),
        }
    }
    pub fn expand_backwards(&self, to_start_idx: usize) -> Self {
        assert!(self.start_idx >= to_start_idx);
        LLCursor {
            start_idx: to_start_idx,
            end_idx: self.end_idx,
            ll_line: self.ll_line.clone(),
        }
    }
    pub fn as_selection(self) -> LLSelection {
        LLSelection {
            ll_line: self.ll_line,
            start_idx: self.start_idx,
            end_idx: self.end_idx,
        }
    }
}

#[derive(Clone)]
pub struct LLCursorStart {
    pub(crate) ll_line: Rc<LLLine>,
    /// Where to begin in the line (inclusive, default is 0)
    start_at_idx: usize,
}


impl LLCursorStart {
    // really relaxed, uncomfortably so.
    pub fn find_start_eq<T: 'static + PartialEq>(&self, equals_attr: &T) -> Vec<LLCursor> {
        self.ll_line
            .attrs
            .values
            .iter()
            .filter_map(|(&(start_idx, end_idx), value)| {
                let attrs = value.get::<T>();
                if !attrs.is_empty() {
                    Some(
                        attrs
                            .iter()
                            .filter(|attr| *attr == equals_attr)
                            .map(move |_| LLCursor {
                                start_idx,
                                end_idx,
                                ll_line: self.ll_line.clone(),
                            }),
                    )
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    // really relaxed, uncomfortably so.
    pub fn find_start_tag(&self, find_tag: &TextTag) -> Vec<(LLCursor, &str)> {
        self.ll_line.ll_tokens[self.start_at_idx..]
            .iter()
            .filter_map(|ll_token| {
                if let LToken::Text(text, tag) = &ll_token.token {
                    if tag == find_tag {
                        Some((
                            LLCursor {
                                start_idx: ll_token.token_idx,
                                end_idx: ll_token.token_idx,
                                ll_line: self.ll_line.clone(),
                            },
                            text.as_str(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_next_start_tag(&self, find_tag: &TextTag) -> Option<(LLCursor, &str)> {
        // Not optimal, but okay for now
        self.find_start_tag(find_tag).into_iter().next()
    }

    pub fn find_next_eq<Attr: 'static + std::fmt::Debug + PartialEq>(
        &self,
        value: &Attr,
    ) -> Option<LLCursor> {
        // Not optimal, but okay for now
        self.find_start_eq(value).into_iter().next()
    }

    pub fn find_start<Attr: 'static + std::fmt::Debug>(&self) -> Vec<(LLCursor, &Attr)> {
        self.ll_line
            .attrs
            .values
            .iter()
            .filter_map(|(&(start, end), value)| {
                let attrs = value.get::<Attr>();
                if !attrs.is_empty() {
                    Some(attrs.iter().map(move |attr| {
                        (
                            LLCursor {
                                start_idx: start,
                                end_idx: end,
                                ll_line: self.ll_line.clone(),
                            },
                            attr,
                        )
                    }))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
}

pub enum MatchResult {
    Some(LLCursor, ()),
    None,
    EndOfLine(LLCursor, ()),
}

pub enum FoundOrEndOfLine<'a, T> {
    Found(LLCursor, &'a T),
    EndOfLine(LLCursor),
}

impl LLCursor {
    pub fn start_after(&self) -> LLCursorStart {
        LLCursorStart {
            ll_line: self.ll_line.clone(),
            start_at_idx: self.end_idx + 1,
        }
    }

    pub fn match_forwards<Attr: 'static>(&self) -> Vec<(LLCursor, &Attr)> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        if self.end_idx + 1 == self.ll_line.ll_tokens.len() {
            return Vec::new();
        }

        self.ll_line
            .attrs
            .starts_at
            .get(self.end_idx + 1)
            .expect("Huh... match_forwards was at the end")
            .get::<Attr>()
            .iter()
            .flat_map(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<Attr>()
                    .iter()
                    .map(move |val| (val, range.1))
            })
            .map(|(val, end_idx)| {
                (
                    LLCursor {
                        start_idx: self.start_idx,
                        end_idx,
                        ll_line: self.ll_line.clone(),
                    },
                    val,
                )
            })
            .collect()
    }
    pub fn match_forwards_char(&self, c: &[char]) -> Option<LLCursor> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        //        [...] - Trying to match Attr
        if self.end_idx + 1 == self.ll_line.ll_tokens.len() {
            return None;
        }

        self.ll_line
            .attrs
            .starts_at
            .get(self.end_idx + 1)
            .expect("Huh... match_forwards_char was at the end")
            .get::<char>()
            .iter()
            .flat_map(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<char>() // probably optimizable since there is only going to ever be one potential char per token
                    .iter()
                    .map(move |val| (val, range.1))
            })
            .filter_map(|(val, end_idx)| {
                if c.contains(&val) {
                    Some(LLCursor {
                        start_idx: self.start_idx,
                        end_idx,
                        ll_line: self.ll_line.clone(),
                    })
                } else {
                    None
                }
            })
            .next()
    }
    pub fn match_forwards_tag(&self, tag: &TextTag) -> Option<(LLCursor, &str)> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        //        [...] - Trying to match Attr
        if self.end_idx + 1 == self.ll_line.ll_tokens.len() {
            return None;
        }

        self.ll_line
            .ll_tokens
            .get(self.end_idx + 1)
            .expect("Huh... match_forwards_tag was at the end")
            .token
            .has_tag(&tag)
            .map(|val| {
                (
                    LLCursor {
                        start_idx: self.start_idx,
                        end_idx: self.end_idx + 1,
                        ll_line: self.ll_line.clone(),
                    },
                    val,
                )
            })
    }
    // expands from current forwards
    pub fn match_forwards_until_end_of_line(&self) -> LLCursor {
        if self.end_idx + 1 == self.ll_line.ll_tokens().len() {
            self.clone()
        } else {
            self.expand_forwards(self.ll_line.ll_tokens().len() - 1)
        }
    }
    // expands from current forwards
    pub fn match_forwards_until_before_eq_or_until_end_of_line<T: 'static + PartialEq>(
        &self,
        equals_attr: &T,
    ) -> MatchResult {
        self.ll_line.attrs.starts_at[self.end_idx..]
            .iter()
            .map(|start_at| start_at.get::<T>())
            .flatten()
            .filter(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .any(|val| val == equals_attr)
            })
            .next()
            .map(|&(start_idx, _)| {
                if start_idx > self.start_idx {
                    MatchResult::Some(self.expand_forwards(start_idx - 1), ())
                } else {
                    MatchResult::None
                }
            })
            // or else return end of line
            .unwrap_or_else(|| {
                MatchResult::EndOfLine(self.expand_forwards(self.ll_line.ll_tokens().len() - 1), ())
            })
    }
    // expands from current forwards
    pub fn match_forwards_until_before_or_until_end_of_line<T: 'static + PartialEq>(
        &self,
    ) -> FoundOrEndOfLine<T> {
        self.ll_line.attrs.starts_at[self.end_idx + 1..]
            .iter()
            .map(|start_at| start_at.get::<T>())
            .flatten()
            .flat_map(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .map(move |val| (val, range.0))
            })
            .next()
            .map(|(attr, start_idx)| {
                FoundOrEndOfLine::Found(self.expand_forwards(start_idx - 1), attr)
            })
            // or else return end of line
            .unwrap_or_else(|| {
                FoundOrEndOfLine::EndOfLine(
                    self.expand_forwards(self.ll_line.ll_tokens().len() - 1),
                )
            })
    }
    pub fn match_backwards<Attr: 'static>(&self) -> Vec<(LLCursor, &Attr)> {
        //        [ ... ] - Current Cursor
        // [ ... ] - Trying to match Attr
        //   [...] - Trying to match Attr
        if self.start_idx == 0 {
            return Vec::new();
        }

        let end_idx = self.start_idx - 1;
        self.ll_line
            .attrs
            .ends_at
            .get(end_idx)
            .expect("Huh... match_backwards was at the start")
            .get::<Attr>()
            .iter()
            .flat_map(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<Attr>()
                    .iter()
                    .map(move |val| (val, range.0))
            })
            .map(|(val, start_idx)| {
                (
                    LLCursor {
                        start_idx,
                        end_idx: self.end_idx,
                        ll_line: self.ll_line.clone(),
                    },
                    val,
                )
            })
            .collect()
    }
    pub fn match_backwards_char(&self, c: &[char]) -> Option<LLCursor> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        //        [...] - Trying to match Attr
        if self.start_idx == 0 {
            return None;
        }

        self.ll_line
            .attrs
            .ends_at
            .get(self.start_idx - 1)
            .expect("Huh... match_backwards was at the start")
            .get::<char>()
            .iter()
            .flat_map(|range| {
                self.ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<char>() // probably optimizable since there is only going to ever be one potential char per token
                    .iter()
                    .rev()
                    .map(move |val| (val, range.0))
            })
            .filter_map(|(val, start_idx)| {
                if c.contains(&val) {
                    Some(LLCursor {
                        start_idx,
                        end_idx: self.end_idx,
                        ll_line: self.ll_line.clone(),
                    })
                } else {
                    None
                }
            })
            .next()
    }
    // fn match_forwards_skip_spaces<Attr>(&self) -> Option<(LLCursor, &Attr)> {
    //     unimplemented!()
    // }
    // fn match_backwards_skip_spaces<Attr>(&self) -> Option<(LLCursor, &Attr)> {
    //     unimplemented!()
    // }
    pub fn finish_with_attr<Attr>(&self, value: Attr) -> LLCursorAssignment<Attr> {
        LLCursorAssignment {
            end_idx: self.end_idx,
            start_idx: self.start_idx,
            value,
        }
    }
}
