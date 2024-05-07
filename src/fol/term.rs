use super::op::{BiOp, TriOp, UniOp};
use crate::fol::{hash::TERMMAP, op::BiOpType};
use giputils::grc::Grc;
use std::ops::Deref;

static mut NUM_VAR: u32 = 0;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Term {
    pub(crate) inner: Grc<TermType>,
}

impl Term {
    #[inline]
    fn new(sort: Sort, term: TermType) -> Self {
        let inner = Grc::new(term.clone());
        TERMMAP.insert(term, &inner, sort);
        Self { inner }
    }

    #[inline]
    pub fn sort(&self) -> Sort {
        TERMMAP.sort(self)
    }

    pub fn bool_const(v: bool) -> Self {
        let term = TermType::Const(Const::Bool(v));
        Self::new(Sort::Bool, term)
    }

    pub fn bv_const(bv: &[bool]) -> Self {
        let term = TermType::Const(Const::BV(bv.to_vec()));
        Self::new(Sort::BV(bv.len() as u32), term)
    }

    pub fn new_var(mut sort: Sort) -> Self {
        if let Sort::BV(w) = sort {
            assert!(w > 0);
            if w == 1 {
                sort = Sort::Bool;
            }
        }
        let term = TermType::Var(unsafe { NUM_VAR });
        unsafe { NUM_VAR += 1 };
        Self::new(sort, term)
    }
}

impl Term {
    #[inline]
    pub fn biop(&self, other: &Self, op: BiOpType, sort: Sort) -> Self {
        let term = TermType::BiOp(BiOp {
            ty: op,
            a: self.clone(),
            b: other.clone(),
        });
        if let Some(inner) = TERMMAP.get(&term) {
            Self { inner }
        } else {
            Self::new(sort, term)
        }
    }

    #[inline]
    pub fn equal(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::Eq, Sort::Bool)
    }

    #[inline]
    pub fn not_equal(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::Neq, Sort::Bool)
    }

    #[inline]
    pub fn and(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::And, self.sort())
    }

    #[inline]
    pub fn or(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::Or, self.sort())
    }

    #[inline]
    pub fn add(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::Add, self.sort())
    }
}

impl Deref for Term {
    type Target = TermType;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl Drop for Term {
    #[inline]
    fn drop(&mut self) {
        if self.inner.count() == 1 {
            TERMMAP.remove(&self.inner);
        }
    }
}

unsafe impl Sync for Term {}

unsafe impl Send for Term {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TermType {
    Const(Const),
    Var(u32),
    UniOp(UniOp),
    BiOp(BiOp),
    TriOp(TriOp),
}

unsafe impl Sync for TermType {}

unsafe impl Send for TermType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sort {
    Bool,
    BV(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Const {
    Bool(bool),
    BV(Vec<bool>),
}
