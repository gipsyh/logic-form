use super::{
    op::{BiOp, TriOp, UniOp},
    ExtOp, ExtOpType, TriOpType, UniOpType,
};
use crate::fol::{hash::TERMMAP, op::BiOpType};
use giputils::grc::Grc;
use std::{
    cmp::Ordering,
    fmt::{self, Debug},
    ops::Deref,
};

static mut NUM_VAR: u32 = 0;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Term {
    pub(crate) inner: Grc<TermType>,
}

impl Term {
    #[inline]
    fn new(sort: Sort, term: TermType) -> Self {
        if let Some(inner) = TERMMAP.get(&term) {
            return Self { inner };
        }
        let inner = Grc::new(term.clone());
        TERMMAP.insert(term, &inner, sort);
        Self { inner }
    }

    #[inline]
    pub fn term_id(&self) -> usize {
        self.inner.as_ptr() as _
    }

    #[inline]
    pub fn sort(&self) -> Sort {
        TERMMAP.sort(self)
    }

    #[inline]
    pub fn bool_const(v: bool) -> Self {
        let term = TermType::Const(Const::Bool(v));
        Self::new(Sort::Bool, term)
    }

    #[inline]
    pub fn bv_const(bv: &[bool]) -> Self {
        let term = TermType::Const(Const::BV(bv.to_vec()));
        Self::new(Sort::BV(bv.len() as u32), term)
    }

    #[inline]
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
    pub fn uniop(&self, op: UniOpType) -> Self {
        let sort = self.sort();
        let term = TermType::UniOp(UniOp {
            ty: op,
            a: self.clone(),
        });
        Self::new(sort, term)
    }

    #[inline]
    pub fn not(&self) -> Self {
        self.uniop(UniOpType::Not)
    }

    #[inline]
    pub fn biop(&self, other: &Self, op: BiOpType) -> Self {
        let sort = self.sort();
        let term = TermType::BiOp(BiOp {
            ty: op,
            a: self.clone(),
            b: other.clone(),
        });
        Self::new(sort, term)
    }

    #[inline]
    pub fn equal(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::Eq)
    }

    #[inline]
    pub fn not_equal(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::Neq)
    }

    #[inline]
    pub fn and(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::And)
    }

    #[inline]
    pub fn or(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::Or)
    }

    #[inline]
    pub fn add(&self, other: &Self) -> Self {
        self.biop(other, BiOpType::Add)
    }

    #[inline]
    pub fn triop(&self, x: &Self, y: &Self, op: TriOpType) -> Self {
        let sort = self.sort();
        let term = TermType::TriOp(TriOp {
            ty: op,
            a: self.clone(),
            b: x.clone(),
            c: y.clone(),
        });
        Self::new(sort, term)
    }

    #[inline]
    pub fn extop(&self, op: ExtOpType, length: u32) -> Self {
        let sort = Sort::BV(
            length
                + match self.sort() {
                    Sort::Bool => 1,
                    Sort::BV(w) => w,
                },
        );
        let term = TermType::ExtOp(ExtOp {
            ty: op,
            a: self.clone(),
            length,
        });
        Self::new(sort, term)
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
            self.inner.increment_count();
            // TERMMAP.remove(&self.inner);
        }
    }
}

impl PartialOrd for Term {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.term_id().partial_cmp(&other.term_id())
    }
}

impl Ord for Term {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.term_id().cmp(&other.term_id())
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
    ExtOp(ExtOp),
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

#[derive(Clone, Default, PartialEq, Eq)]
pub struct TermCube {
    cube: Vec<Term>,
}

impl Debug for TermCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cube.fmt(f)
    }
}

impl TermCube {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.cube.len()
    }

    #[inline]
    pub fn push(&mut self, term: Term) {
        let Err(index) = self.cube.binary_search(&term) else {
            todo!()
        };
        self.cube.insert(index, term);
        assert!(self.cube.is_sorted());
    }

    pub fn term(&self) -> Term {
        let mut term = Term::bool_const(true);
        for l in self.iter() {
            term = term.and(l);
        }
        term
    }
}

impl Deref for TermCube {
    type Target = [Term];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.cube
    }
}
