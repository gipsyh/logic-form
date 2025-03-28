#![feature(step_trait)]

mod cnf;
mod dagcnf;
pub mod dimacs;
mod lbool;
mod lemma;
mod litvec;
mod litvvec;
mod utils;

pub use cnf::*;
pub use dagcnf::*;
pub use lbool::*;
pub use lemma::*;
pub use litvec::*;
pub use litvvec::*;
pub use utils::*;

use std::{
    cmp,
    fmt::{self, Debug, Display},
    hash::Hash,
    iter::Step,
    ops::{Add, AddAssign, Deref, Not},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Var(pub u32);

impl Var {
    pub const CONST: Var = Var(0);

    #[inline]
    pub fn new(x: usize) -> Self {
        Self(x as _)
    }

    #[inline]
    pub fn lit(&self) -> Lit {
        Lit(self.0 << 1)
    }

    #[inline]
    pub fn is_constant(&self) -> bool {
        *self == Self::CONST
    }
}

impl Add<Var> for Var {
    type Output = Var;

    #[inline]
    fn add(self, rhs: Var) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Var> for Var {
    #[inline]
    fn add_assign(&mut self, rhs: Var) {
        self.0 += rhs.0;
    }
}

impl Add<u32> for Var {
    type Output = Var;

    #[inline]
    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u32> for Var {
    #[inline]
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

impl From<Lit> for Var {
    #[inline]
    fn from(value: Lit) -> Self {
        value.var()
    }
}

impl From<u32> for Var {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<i32> for Var {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value as u32)
    }
}

impl From<usize> for Var {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<Var> for u32 {
    #[inline]
    fn from(value: Var) -> Self {
        value.0
    }
}

impl From<Var> for i32 {
    #[inline]
    fn from(value: Var) -> Self {
        value.0 as i32
    }
}

impl From<Var> for usize {
    #[inline]
    fn from(value: Var) -> Self {
        value.0 as usize
    }
}

impl Deref for Var {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Var {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Step for Var {
    #[inline]
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        u32::steps_between(&start.0, &end.0)
    }

    #[inline]
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        u32::forward_checked(start.0, count).map(Self)
    }

    #[inline]
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        u32::backward_checked(start.0, count).map(Self)
    }
}

impl PartialEq<u32> for Var {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        (self.0 as u32).eq(other)
    }
}

impl PartialOrd<u32> for Var {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<cmp::Ordering> {
        (self.0 as u32).partial_cmp(other)
    }
}

impl PartialEq<usize> for Var {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        (self.0 as usize).eq(other)
    }
}

impl PartialOrd<usize> for Var {
    #[inline]
    fn partial_cmp(&self, other: &usize) -> Option<cmp::Ordering> {
        (self.0 as usize).partial_cmp(other)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Lit(u32);

impl From<Var> for Lit {
    #[inline]
    fn from(value: Var) -> Self {
        Self(value.0 << 1)
    }
}

impl From<Lit> for u32 {
    #[inline]
    fn from(val: Lit) -> Self {
        val.0
    }
}

impl From<Lit> for i32 {
    #[inline]
    fn from(val: Lit) -> Self {
        let mut v: i32 = val.var().into();
        if !val.polarity() {
            v = -v;
        }
        v
    }
}

impl From<i32> for Lit {
    #[inline]
    fn from(value: i32) -> Self {
        Self::new(Var(value.unsigned_abs()), value > 0)
    }
}

impl Lit {
    #[inline]
    pub fn new(var: Var, polarity: bool) -> Self {
        Lit(var.0 + var.0 + !polarity as u32)
    }

    #[inline]
    pub fn var(&self) -> Var {
        Var(self.0 >> 1)
    }

    #[inline]
    pub fn polarity(&self) -> bool {
        self.0 & 1 == 0
    }

    #[inline]
    pub fn constant(polarity: bool) -> Self {
        Self::new(Var::CONST, !polarity)
    }

    #[inline]
    pub fn is_constant(&self, polarity: bool) -> bool {
        *self == Self::constant(polarity)
    }

    #[inline]
    pub fn not_if(&self, c: bool) -> Self {
        if c { !*self } else { *self }
    }

    #[inline]
    pub fn cube(&self) -> LitVec {
        LitVec::from([*self])
    }
}

impl Not for Lit {
    type Output = Self;

    #[inline]
    fn not(mut self) -> Self::Output {
        self.0 ^= 1;
        self
    }
}

impl Debug for Lit {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.polarity() {
            write!(f, "{}", self.var())
        } else {
            write!(f, "-{}", self.var())
        }
    }
}

impl Display for Lit {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}
