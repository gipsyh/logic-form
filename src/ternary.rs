use std::{
    fmt::{self, Display, Formatter},
    ops::{BitAnd, BitOr, Not},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TernaryValue {
    True,
    False,
    X,
}

impl Default for TernaryValue {
    fn default() -> Self {
        Self::X
    }
}

impl Not for TernaryValue {
    type Output = TernaryValue;

    fn not(self) -> Self::Output {
        match self {
            TernaryValue::True => TernaryValue::False,
            TernaryValue::False => TernaryValue::True,
            TernaryValue::X => TernaryValue::X,
        }
    }
}

impl BitAnd for TernaryValue {
    type Output = TernaryValue;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TernaryValue::True, TernaryValue::True) => TernaryValue::True,
            (TernaryValue::True, TernaryValue::False) => TernaryValue::False,
            (TernaryValue::True, TernaryValue::X) => TernaryValue::X,
            (TernaryValue::False, TernaryValue::True) => TernaryValue::False,
            (TernaryValue::False, TernaryValue::False) => TernaryValue::False,
            (TernaryValue::False, TernaryValue::X) => TernaryValue::False,
            (TernaryValue::X, TernaryValue::True) => TernaryValue::X,
            (TernaryValue::X, TernaryValue::False) => TernaryValue::False,
            (TernaryValue::X, TernaryValue::X) => TernaryValue::X,
        }
    }
}

impl BitOr for TernaryValue {
    type Output = TernaryValue;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TernaryValue::True, TernaryValue::True) => TernaryValue::True,
            (TernaryValue::True, TernaryValue::False) => TernaryValue::True,
            (TernaryValue::True, TernaryValue::X) => TernaryValue::True,
            (TernaryValue::False, TernaryValue::True) => TernaryValue::True,
            (TernaryValue::False, TernaryValue::False) => TernaryValue::False,
            (TernaryValue::False, TernaryValue::X) => TernaryValue::X,
            (TernaryValue::X, TernaryValue::True) => TernaryValue::True,
            (TernaryValue::X, TernaryValue::False) => TernaryValue::X,
            (TernaryValue::X, TernaryValue::X) => TernaryValue::X,
        }
    }
}

impl TernaryValue {
    pub fn not_if(self, x: bool) -> Self {
        if x {
            !self
        } else {
            self
        }
    }
}

impl From<bool> for TernaryValue {
    fn from(value: bool) -> Self {
        if value {
            TernaryValue::True
        } else {
            TernaryValue::False
        }
    }
}

impl From<char> for TernaryValue {
    #[inline]
    fn from(value: char) -> Self {
        match value {
            '1' => Self::True,
            '0' => Self::False,
            'x' => Self::X,
            'X' => Self::X,
            _ => panic!(),
        }
    }
}

impl Display for TernaryValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TernaryValue::True => '1'.fmt(f),
            TernaryValue::False => '0'.fmt(f),
            TernaryValue::X => 'X'.fmt(f),
        }
    }
}
