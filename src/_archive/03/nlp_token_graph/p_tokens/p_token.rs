/// [PToken] is different from [ui::Token], [ecsEcsToken], [view::ViewToken], etc. as it is the smallest part of a token line.
///
/// This means that an [PToken], should never have a space in it or have both letters and puntuation
pub enum PTokenKind {
    Text(std::borrow::Cow<'static, str>),
    // info about atom
    Atom {},
    // info about expression
    Expression {},
}

fn text(s: &str) -> PTokenKind {
    PTokenKind::Text(s.to_string().into())
}

/// S: "Start"
/// B: "$"
/// C: "10"
/// B': "$10" + CurrencyDetection(USDPennies, 1000)
///  S_B_C_D_
///   \_B'_/
pub struct PToken {
    kind: PTokenKind,
    /// This token's index into it's [super::PTokens] container.
    index_at: super::PTokenAt,
    /// The position directly after this token
    index_to: super::PTokenRangeTo,
    // /// Fine-grained individual token information ?
    // attributes: TypeMap,
    /// Where the token's position start is
    from_position: usize,
    /// Where the last token's position end is
    to_position: usize,
}

impl PToken {
    /// Get a reference to the n l p token's kind.
    pub fn kind(&self) -> &PTokenKind {
        &self.kind
    }

    /// Get a reference to the n l p token's from position.
    pub fn from_position(&self) -> &usize {
        &self.from_position
    }

    /// Get a reference to the n l p token's to position.
    pub fn to_position(&self) -> &usize {
        &self.to_position
    }

    /// Get a reference to the n l p span token's token index location.
    pub fn token_index_at(&self) -> &super::PTokenAt {
        &self.index_at
    }

    /// Get a reference to the n l p span token's index to which can be used to generate ranges.
    pub fn token_index_to(&self) -> &super::PTokenRangeTo {
        &self.index_to
    }
}
