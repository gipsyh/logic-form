use super::TermVec;
use super::op::{Add, And, Ite, Neg, Not, Or, Sub, Xor};
use super::{op::DynOp, sort::Sort};
use crate::fol::op::Slice;
use giputils::grc::Grc;
use giputils::hash::GHashMap;
use std::fmt::{self, Debug};
use std::iter::once;
use std::ops::{ControlFlow, DerefMut, FromResidual, Index, Try};
use std::{hash, ops};
use std::{hash::Hash, ops::Deref};

#[derive(Clone)]
pub struct Term {
    tm: TermManager,
    pub(crate) inner: Grc<TermInner>,
}

impl Term {
    #[inline]
    pub fn get_tm(&self) -> TermManager {
        self.tm.clone()
    }

    #[inline]
    pub fn sort(&self) -> Sort {
        self.inner.sort()
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        self.sort().is_bool()
    }

    #[inline]
    pub fn is_const(&self) -> bool {
        matches!(self.deref(), TermType::Const(_))
    }

    #[inline]
    pub fn try_op_term(&self) -> Option<&OpTerm> {
        if let TermType::Op(op) = self.deref() {
            Some(op)
        } else {
            None
        }
    }

    #[inline]
    pub fn try_var_term(&self) -> Option<u32> {
        if let TermType::Var(v) = self.deref() {
            Some(*v)
        } else {
            None
        }
    }

    #[inline]
    pub fn var_term(&self) -> u32 {
        self.try_var_term().unwrap()
    }

    #[inline]
    pub fn bv_len(&self) -> usize {
        self.sort().bv()
    }

    #[inline]
    pub fn try_bv_const(&self) -> Option<&BvConst> {
        match self.deref() {
            TermType::Const(c) => Some(c),
            _ => None,
        }
    }

    #[inline]
    pub fn mk_bv_const_zero(&self) -> Term {
        let mut tm = self.get_tm();
        tm.bv_const_zero(self.bv_len())
    }

    #[inline]
    pub fn mk_bv_const_one(&self) -> Term {
        let mut tm = self.get_tm();
        tm.bv_const_one(self.bv_len())
    }

    #[inline]
    pub fn mk_bv_const_ones(&self) -> Term {
        let mut tm = self.get_tm();
        tm.bv_const_ones(self.bv_len())
    }

    #[inline]
    pub fn op<'a>(
        &'a self,
        op: impl Into<DynOp>,
        terms: impl IntoIterator<Item = impl AsRef<Term> + 'a>,
    ) -> Term {
        let mut tm = self.get_tm();
        let terms = once(self.clone()).chain(terms.into_iter().map(|l| l.as_ref().clone()));
        tm.new_op_term(op.into(), terms)
    }

    #[inline]
    pub fn op0(&self, op: impl Into<DynOp>) -> Term {
        let mut tm = self.get_tm();
        tm.new_op_term(op.into(), [self])
    }

    #[inline]
    pub fn op1(&self, op: impl Into<DynOp>, x: &Term) -> Term {
        let mut tm = self.get_tm();
        tm.new_op_term(op.into(), [self, x])
    }

    #[inline]
    pub fn op2(&self, op: impl Into<DynOp>, x: &Term, y: &Term) -> Term {
        let mut tm = self.get_tm();
        tm.new_op_term(op.into(), [self, x, y])
    }

    #[inline]
    pub fn not_if(&self, c: bool) -> Term {
        if c { !self } else { self.clone() }
    }

    #[inline]
    pub fn ite(&self, t: &Term, e: &Term) -> Term {
        self.op2(Ite, t, e)
    }

    pub fn slice(&self, l: usize, h: usize) -> Term {
        let mut tm = self.get_tm();
        let h = tm.bv_const_zero(h);
        let l = tm.bv_const_zero(l);
        self.op2(Slice, &h, &l)
    }
}

impl Deref for Term {
    type Target = TermType;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner.ty
    }
}

impl Hash for Term {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl Debug for Term {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.deref().fmt(f)
    }
}

impl<T: AsRef<Term>> PartialEq<T> for Term {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        let other = other.as_ref();
        debug_assert!(self.tm == other.tm);
        self.inner == other.inner
    }
}

impl Eq for Term {}

impl AsRef<Term> for Term {
    #[inline]
    fn as_ref(&self) -> &Term {
        self
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let g = self.clone();
        self.tm.tgc.collect(g);
    }
}

macro_rules! impl_unary_ops {
    ($trait:ident, $method:ident, $op:expr) => {
        impl std::ops::$trait for Term {
            type Output = Term;

            #[inline]
            fn $method(self) -> Self::Output {
                self.op0($op)
            }
        }

        impl std::ops::$trait for &Term {
            type Output = Term;

            #[inline]
            fn $method(self) -> Self::Output {
                self.op0($op)
            }
        }
    };
}

impl_unary_ops!(Not, not, Not);
impl_unary_ops!(Neg, neg, Neg);

macro_rules! impl_biops {
    ($trait:ident, $method:ident, $op:expr) => {
        impl<T: AsRef<Term>> std::ops::$trait<T> for Term {
            type Output = Term;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                self.op1($op, rhs.as_ref())
            }
        }

        impl<T: AsRef<Term>> std::ops::$trait<T> for &Term {
            type Output = Term;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                self.op1($op, rhs.as_ref())
            }
        }
    };
}

impl_biops!(BitAnd, bitand, And);
impl_biops!(BitOr, bitor, Or);
impl_biops!(BitXor, bitxor, Xor);
impl_biops!(Add, add, Add);
impl_biops!(Sub, sub, Sub);

pub struct TermInner {
    sort: Sort,
    ty: TermType,
}

impl TermInner {
    #[inline]
    pub fn sort(&self) -> Sort {
        self.sort
    }
}

impl Debug for TermInner {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ty {
            TermType::Const(c) => c.fmt(f),
            TermType::Var(v) => write!(f, "Var{}, {:?}", *v, self.sort),
            TermType::Op(o) => o.fmt(f),
        }
    }
}

impl Deref for TermInner {
    type Target = TermType;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TermType {
    Const(BvConst),
    Var(u32),
    Op(OpTerm),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BvConst {
    c: Vec<bool>,
}

impl BvConst {
    #[inline]
    pub fn new(c: &[bool]) -> Self {
        Self { c: c.to_vec() }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.c.iter().all(|x| !x)
    }

    #[inline]
    pub fn is_ones(&self) -> bool {
        self.c.iter().all(|x| *x)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.c.len()
    }

    pub fn bool(&self) -> Option<bool> {
        if self.c.len() == 1 {
            Some(self.c[0])
        } else {
            None
        }
    }
}

impl Deref for BvConst {
    type Target = [bool];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}

impl DerefMut for BvConst {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.c
    }
}

impl Debug for BvConst {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c: String = self
            .c
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .rev()
            .collect();
        write!(f, "BvConst({c:})")
    }
}

impl ops::Not for &BvConst {
    type Output = BvConst;

    #[inline]
    fn not(self) -> Self::Output {
        let c = self.c.iter().map(|b| !b).collect();
        BvConst { c }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct OpTerm {
    pub op: DynOp,
    pub terms: Vec<Term>,
}

impl OpTerm {
    #[inline]
    fn new(op: impl Into<DynOp>, terms: Vec<Term>) -> Self {
        Self {
            op: op.into(),
            terms: terms.to_vec(),
        }
    }
}

impl Debug for OpTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.op.fmt(f)?;
        self.terms.fmt(f)
    }
}

impl Index<usize> for OpTerm {
    type Output = Term;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.terms[index]
    }
}

pub enum TermResult {
    Some(Term),
    None,
}

impl FromResidual for TermResult {
    #[inline]
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        TermResult::Some(residual)
    }
}

impl Try for TermResult {
    type Output = ();

    type Residual = Term;

    #[inline]
    fn from_output(_: Self::Output) -> Self {
        TermResult::None
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            TermResult::Some(term) => ControlFlow::Break(term),
            TermResult::None => ControlFlow::Continue(()),
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct TermGC {
    garbage: Grc<Vec<Term>>,
}

impl TermGC {
    #[inline]
    pub fn collect(&mut self, term: Term) {
        self.garbage.push(term);
    }
}

#[derive(Default)]
pub struct TermManagerInner {
    tgc: TermGC,
    num_var: u32,
    map: GHashMap<TermType, Term>,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct TermManager {
    inner: Grc<TermManagerInner>,
}

impl TermManager {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn new_term(&mut self, ty: TermType, sort: Sort) -> Term {
        match self.map.get(&ty) {
            Some(term) => term.clone(),
            None => {
                let term = Term {
                    tm: self.clone(),
                    inner: Grc::new(TermInner {
                        sort,
                        ty: ty.clone(),
                    }),
                };
                self.map.insert(ty, term.clone());
                term
            }
        }
    }

    #[inline]
    pub fn bool_const(&mut self, c: bool) -> Term {
        let term = TermType::Const(BvConst::new(&[c]));
        self.new_term(term, Sort::Bv(1))
    }

    #[inline]
    pub fn bv_const(&mut self, c: BvConst) -> Term {
        let sort = Sort::Bv(c.len());
        let term = TermType::Const(c);
        self.new_term(term, sort)
    }

    #[inline]
    pub fn bv_const_zero(&mut self, len: usize) -> Term {
        let c = vec![false; len];
        let term = TermType::Const(BvConst::new(&c));
        self.new_term(term, Sort::Bv(len))
    }

    #[inline]
    pub fn bv_const_one(&mut self, len: usize) -> Term {
        let mut c = vec![false; len];
        c[0] = true;
        let term = TermType::Const(BvConst::new(&c));
        self.new_term(term, Sort::Bv(len))
    }

    #[inline]
    pub fn bv_const_ones(&mut self, len: usize) -> Term {
        let c = vec![true; len];
        let term = TermType::Const(BvConst::new(&c));
        self.new_term(term, Sort::Bv(len))
    }

    #[inline]
    pub fn bv_const_from_usize(&mut self, mut v: usize, width: usize) -> Term {
        let mut bv = Vec::new();
        while v > 0 {
            bv.push(width & 1 == 1);
            v >>= 1;
        }
        while bv.len() < width {
            bv.push(false);
        }
        bv.truncate(width);
        self.bv_const(BvConst::new(&bv))
    }

    #[inline]
    pub fn new_op_term(
        &mut self,
        op: impl Into<DynOp>,
        terms: impl IntoIterator<Item = impl AsRef<Term>>,
    ) -> Term {
        let op: DynOp = op.into();
        let terms: Vec<Term> = terms.into_iter().map(|t| t.as_ref().clone()).collect();
        if !op.is_core() {
            return op.normalize(&terms);
        }
        let sort = op.sort(&terms);
        let term = TermType::Op(OpTerm::new(op, terms));
        self.new_term(term, sort)
    }

    #[inline]
    pub fn new_op_terms_fold(
        &mut self,
        op: impl Into<DynOp> + Copy,
        terms: impl IntoIterator<Item = impl AsRef<Term>>,
    ) -> Term {
        let mut terms = terms.into_iter();
        let acc = terms.next().unwrap().as_ref().clone();
        terms.fold(acc, |acc, x| {
            self.new_op_term(op, &[acc, x.as_ref().clone()])
        })
    }

    #[inline]
    pub fn new_op_terms_elementwise<'a>(
        &mut self,
        op: impl Into<DynOp> + Copy,
        x: impl IntoIterator<Item = &'a Term>,
        y: impl IntoIterator<Item = &'a Term>,
    ) -> TermVec {
        x.into_iter()
            .zip(y)
            .map(|(x, y)| self.new_op_term(op, [x, y]))
            .collect()
    }

    #[inline]
    pub fn new_var(&mut self, sort: Sort) -> Term {
        let id = self.num_var;
        self.num_var += 1;
        let term = TermType::Var(id);
        self.new_term(term, sort)
    }

    #[inline]
    pub fn garbage_collect(&mut self) {}

    #[inline]
    pub fn size(&self) -> usize {
        self.map.len()
    }
}

impl Deref for TermManager {
    type Target = TermManagerInner;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TermManager {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Debug for TermManager {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TermManager")
            .field("size", &self.size())
            .finish()
    }
}

// impl PartialOrd for TermCube {
//     #[inline]
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for TermCube {
//     #[inline]
//     fn cmp(&self, other: &Self) -> Ordering {
//         debug_assert!(self.is_sorted());
//         debug_assert!(other.is_sorted());
//         let min_index = self.len().min(other.len());
//         for i in 0..min_index {
//             match self[i].cmp(&other[i]) {
//                 Ordering::Less => return Ordering::Less,
//                 Ordering::Equal => {}
//                 Ordering::Greater => return Ordering::Greater,
//             }
//         }
//         self.len().cmp(&other.len())
//     }
// }

// impl FromIterator<Term> for TermCube {
//     #[inline]
//     fn from_iter<T: IntoIterator<Item = Term>>(iter: T) -> Self {
//         Self {
//             cube: Vec::from_iter(iter),
//         }
//     }
// }
