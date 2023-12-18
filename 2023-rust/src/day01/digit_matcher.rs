pub struct DigitMatcher {
    pub allow_spelled_out: bool,
    state: State,
}

impl DigitMatcher {
    pub fn new(allow_spelled_out: bool) -> Self {
        Self {
            allow_spelled_out,
            state: State::Initial,
        }
    }

    pub fn update(&mut self, ch: char) -> Option<u8> {
        let (new_state, digit) = self.state.update(ch);
        self.state = new_state;
        digit.to_opt(self.allow_spelled_out)
    }
}

#[derive(Clone, Copy)]
enum Digit {
    Numeric(u8),
    SpelledOut(u8),
    None,
}

impl Digit {
    fn to_opt(self, allow_spelled_out: bool) -> Option<u8> {
        match self {
            Self::Numeric(n) => Some(n),
            Self::SpelledOut(n) => {
                if allow_spelled_out {
                    Some(n)
                } else {
                    None
                }
            }
            Self::None => None,
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    Initial,
    MatchedE,
    MatchedF,
    MatchedN,
    MatchedO,
    MatchedS,
    MatchedT,
    MatchedZ,
    MatchedEi,
    MatchedFi,
    MatchedFo,
    MatchedNi,
    MatchedOn,
    MatchedSe,
    MatchedSi,
    MatchedTh,
    MatchedTw,
    MatchedZe,
    MatchedEig,
    MatchedFiv,
    MatchedFou,
    MatchedNin,
    MatchedSev,
    MatchedThr,
    MatchedZer,
    MatchedEigh,
    MatchedSeve,
    MatchedThre,
}

impl State {
    fn update(self, ch: char) -> (Self, Digit) {
        if ch.is_digit(10) {
            return (Self::Initial, Digit::Numeric((ch as u8) - ('0' as u8)));
        }

        match (self, ch) {
            (Self::MatchedE, 'i') => (Self::MatchedEi, Digit::None),
            (Self::MatchedF, 'i') => (Self::MatchedFi, Digit::None),
            (Self::MatchedF, 'o') => (Self::MatchedFo, Digit::None),
            (Self::MatchedN, 'i') => (Self::MatchedNi, Digit::None),
            (Self::MatchedO, 'n') => (Self::MatchedOn, Digit::None),
            (Self::MatchedS, 'e') => (Self::MatchedSe, Digit::None),
            (Self::MatchedS, 'i') => (Self::MatchedSi, Digit::None),
            (Self::MatchedT, 'h') => (Self::MatchedTh, Digit::None),
            (Self::MatchedT, 'w') => (Self::MatchedTw, Digit::None),
            (Self::MatchedZ, 'e') => (Self::MatchedZe, Digit::None),
            (Self::MatchedEi, 'g') => (Self::MatchedEig, Digit::None),
            (Self::MatchedFi, 'v') => (Self::MatchedFiv, Digit::None),
            (Self::MatchedFo, 'n') => (Self::MatchedOn, Digit::None),
            (Self::MatchedFo, 'u') => (Self::MatchedFou, Digit::None),
            (Self::MatchedNi, 'n') => (Self::MatchedNin, Digit::None),
            (Self::MatchedOn, 'e') => (Self::MatchedE, Digit::SpelledOut(1)),
            (Self::MatchedOn, 'i') => (Self::MatchedNi, Digit::None),
            (Self::MatchedSe, 'i') => (Self::MatchedEi, Digit::None),
            (Self::MatchedSe, 'v') => (Self::MatchedSev, Digit::None),
            (Self::MatchedSi, 'x') => (Self::Initial, Digit::SpelledOut(6)),
            (Self::MatchedTh, 'r') => (Self::MatchedThr, Digit::None),
            (Self::MatchedTw, 'o') => (Self::MatchedO, Digit::SpelledOut(2)),
            (Self::MatchedZe, 'i') => (Self::MatchedEi, Digit::None),
            (Self::MatchedZe, 'r') => (Self::MatchedZer, Digit::None),
            (Self::MatchedEig, 'h') => (Self::MatchedEigh, Digit::None),
            (Self::MatchedFiv, 'e') => (Self::MatchedE, Digit::SpelledOut(5)),
            (Self::MatchedFou, 'r') => (Self::Initial, Digit::SpelledOut(4)),
            (Self::MatchedNin, 'e') => (Self::MatchedE, Digit::SpelledOut(9)),
            (Self::MatchedNin, 'i') => (Self::MatchedNi, Digit::None),
            (Self::MatchedSev, 'e') => (Self::MatchedSeve, Digit::None),
            (Self::MatchedThr, 'e') => (Self::MatchedThre, Digit::None),
            (Self::MatchedZer, 'o') => (Self::MatchedO, Digit::SpelledOut(0)),
            (Self::MatchedEigh, 't') => (Self::MatchedT, Digit::SpelledOut(8)),
            (Self::MatchedSeve, 'i') => (Self::MatchedEi, Digit::None),
            (Self::MatchedSeve, 'n') => (Self::MatchedN, Digit::SpelledOut(7)),
            (Self::MatchedThre, 'e') => (Self::MatchedE, Digit::SpelledOut(3)),
            (Self::MatchedThre, 'i') => (Self::MatchedEi, Digit::None),
            (_, 'e') => (Self::MatchedE, Digit::None),
            (_, 'f') => (Self::MatchedF, Digit::None),
            (_, 'n') => (Self::MatchedN, Digit::None),
            (_, 'o') => (Self::MatchedO, Digit::None),
            (_, 's') => (Self::MatchedS, Digit::None),
            (_, 't') => (Self::MatchedT, Digit::None),
            (_, 'z') => (Self::MatchedZ, Digit::None),
            _ => (Self::Initial, Digit::None),
        }
    }
}
