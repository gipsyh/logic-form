#![feature(step_trait)]

mod cnf;
pub mod dimacs;
mod lbool;
mod utils;

use ahash::AHasher;
pub use cnf::*;
use giputils::hash::GHashSet;
pub use lbool::*;
pub use utils::*;

use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display},
    hash::{Hash, Hasher},
    iter::Step,
    ops::{Add, AddAssign, Deref, DerefMut, Not},
    slice,
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
        if c {
            !*self
        } else {
            *self
        }
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LitVec {
    lits: Vec<Lit>,
}

impl LitVec {
    #[inline]
    pub fn new() -> Self {
        LitVec { lits: Vec::new() }
    }

    #[inline]
    pub fn new_with(c: usize) -> Self {
        LitVec {
            lits: Vec::with_capacity(c),
        }
    }

    #[inline]
    pub fn subsume(&self, o: &[Lit]) -> bool {
        if self.len() > o.len() {
            return false;
        }
        'n: for x in self.iter() {
            for y in o.iter() {
                if x == y {
                    continue 'n;
                }
            }
            return false;
        }
        true
    }

    pub fn subsume_execpt_one(&self, o: &[Lit]) -> (bool, Option<Lit>) {
        if self.len() > o.len() {
            return (false, None);
        }
        let mut diff = None;
        'n: for x in self.iter() {
            for y in o.iter() {
                if x == y {
                    continue 'n;
                }
                if diff.is_none() && x.var() == y.var() {
                    diff = Some(*x);
                    continue 'n;
                }
            }
            return (false, None);
        }

        (diff.is_none(), diff)
    }

    #[inline]
    pub fn ordered_subsume(&self, cube: &LitVec) -> bool {
        debug_assert!(self.is_sorted_by_key(|l| l.var()));
        debug_assert!(cube.is_sorted_by_key(|l| l.var()));
        if self.len() > cube.len() {
            return false;
        }
        let mut j = 0;
        for i in 0..self.len() {
            while j < cube.len() && self[i].0 > cube[j].0 {
                j += 1;
            }
            if j == cube.len() || self[i] != cube[j] {
                return false;
            }
        }
        true
    }

    #[inline]
    pub fn ordered_subsume_execpt_one(&self, cube: &LitVec) -> (bool, Option<Lit>) {
        debug_assert!(self.is_sorted_by_key(|l| l.var()));
        debug_assert!(cube.is_sorted_by_key(|l| l.var()));
        let mut diff = None;
        if self.len() > cube.len() {
            return (false, None);
        }
        let mut j = 0;
        for i in 0..self.len() {
            while j < cube.len() && self[i].var() > cube[j].var() {
                j += 1;
            }
            if j == cube.len() {
                return (false, None);
            }
            if self[i] != cube[j] {
                if diff.is_none() && self[i].var() == cube[j].var() {
                    diff = Some(self[i]);
                } else {
                    return (false, None);
                }
            }
        }
        (diff.is_none(), diff)
    }

    #[inline]
    pub fn intersection(&self, cube: &LitVec) -> LitVec {
        let x_lit_set = self.iter().collect::<GHashSet<&Lit>>();
        let y_lit_set = cube.iter().collect::<GHashSet<&Lit>>();
        Self {
            lits: x_lit_set
                .intersection(&y_lit_set)
                .copied()
                .copied()
                .collect(),
        }
    }

    #[inline]
    pub fn ordered_intersection(&self, cube: &LitVec) -> LitVec {
        debug_assert!(self.is_sorted_by_key(|l| l.var()));
        debug_assert!(cube.is_sorted_by_key(|l| l.var()));
        let mut res = LitVec::new();
        let mut i = 0;
        for l in self.iter() {
            while i < cube.len() && cube[i] < *l {
                i += 1;
            }
            if i == cube.len() {
                break;
            }
            if *l == cube[i] {
                res.push(*l);
            }
        }
        res
    }

    #[inline]
    pub fn resolvent(&self, other: &LitVec, v: Var) -> Option<LitVec> {
        let (x, y) = if self.len() < other.len() {
            (self, other)
        } else {
            (other, self)
        };
        let mut new = LitVec::new();
        'n: for x in x.iter() {
            if x.var() != v {
                for y in y.iter() {
                    if x.var() == y.var() {
                        if *x == !*y {
                            return None;
                        } else {
                            continue 'n;
                        }
                    }
                }
                new.push(*x);
            }
        }
        new.extend(y.iter().filter(|l| l.var() != v).copied());
        Some(new)
    }

    #[inline]
    pub fn ordered_resolvent(&self, other: &LitVec, v: Var) -> Option<LitVec> {
        debug_assert!(self.is_sorted_by_key(|l| l.var()));
        debug_assert!(other.is_sorted_by_key(|l| l.var()));
        let (x, y) = if self.len() < other.len() {
            (self, other)
        } else {
            (other, self)
        };
        let mut new = LitVec::new_with(self.len() + other.len());
        let (mut i, mut j) = (0, 0);
        while i < x.len() {
            if x[i].var() == v {
                i += 1;
                continue;
            }
            while j < y.len() && y[j].var() < x[i].var() {
                j += 1;
            }
            if j < y.len() && x[i].var() == y[j].var() {
                if x[i] == !y[j] {
                    return None;
                }
            } else {
                new.push(x[i]);
            }
            i += 1;
        }
        new.extend(y.iter().filter(|l| l.var() != v).copied());
        Some(new)
    }
}

impl Default for LitVec {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for LitVec {
    type Target = Vec<Lit>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lits
    }
}

impl DerefMut for LitVec {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lits
    }
}

impl PartialOrd for LitVec {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LitVec {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        debug_assert!(self.is_sorted_by_key(|x| x.var()));
        debug_assert!(other.is_sorted_by_key(|x| x.var()));
        let min_index = self.len().min(other.len());
        for i in 0..min_index {
            match self[i].0.cmp(&other[i].0) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
                Ordering::Greater => return Ordering::Greater,
            }
        }
        self.len().cmp(&other.len())
    }
}

impl Not for LitVec {
    type Output = LitVec;

    #[inline]
    fn not(self) -> Self::Output {
        let lits = self.lits.iter().map(|lit| !*lit).collect();
        LitVec { lits }
    }
}

impl Not for &LitVec {
    type Output = LitVec;

    #[inline]
    fn not(self) -> Self::Output {
        let lits = self.lits.iter().map(|lit| !*lit).collect();
        LitVec { lits }
    }
}

impl<const N: usize> From<[Lit; N]> for LitVec {
    #[inline]
    fn from(s: [Lit; N]) -> Self {
        Self { lits: Vec::from(s) }
    }
}

impl From<&[Lit]> for LitVec {
    #[inline]
    fn from(s: &[Lit]) -> Self {
        Self { lits: Vec::from(s) }
    }
}

impl From<LitVec> for Vec<Lit> {
    #[inline]
    fn from(val: LitVec) -> Self {
        val.lits
    }
}

impl FromIterator<Lit> for LitVec {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Lit>>(iter: T) -> Self {
        Self {
            lits: Vec::from_iter(iter),
        }
    }
}

impl IntoIterator for LitVec {
    type Item = Lit;
    type IntoIter = std::vec::IntoIter<Lit>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.lits.into_iter()
    }
}

impl<'a> IntoIterator for &'a LitVec {
    type Item = &'a Lit;
    type IntoIter = slice::Iter<'a, Lit>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.lits.iter()
    }
}

impl AsRef<[Lit]> for LitVec {
    #[inline]
    fn as_ref(&self) -> &[Lit] {
        self.as_slice()
    }
}

impl Display for LitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lits.fmt(f)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Lemma {
    cube: LitVec,
    sign: u128,
    hash: u64,
}

impl Deref for Lemma {
    type Target = LitVec;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.cube
    }
}

impl DerefMut for Lemma {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cube
    }
}

impl PartialEq for Lemma {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.hash != other.hash || self.sign != other.sign || self.len() != other.len() {
            return false;
        }
        for i in 0..self.cube.len() {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Lemma {}

impl PartialOrd for Lemma {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Lemma {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.cube.cmp(&other.cube)
    }
}

impl Lemma {
    #[inline]
    pub fn new(mut cube: LitVec) -> Self {
        cube.sort();
        let mut sign = 0;
        for l in cube.iter() {
            sign |= 1 << (Into::<u32>::into(*l) % u128::BITS);
        }
        let mut hasher = AHasher::default();
        cube.hash(&mut hasher);
        Self {
            cube,
            sign,
            hash: hasher.finish(),
        }
    }

    #[inline]
    pub fn cube(&self) -> &LitVec {
        &self.cube
    }

    #[inline]
    fn var_sign(&self) -> u128 {
        ((self.sign >> 1) | self.sign) & 113427455640312821154458202477256070485_u128
    }

    #[inline]
    pub fn subsume(&self, other: &Lemma) -> bool {
        if self.cube.len() > other.cube.len() {
            return false;
        }
        if self.sign & other.sign != self.sign {
            return false;
        }
        self.cube.ordered_subsume(&other.cube)
    }

    #[inline]
    pub fn subsume_execpt_one(&self, other: &Lemma) -> (bool, Option<Lit>) {
        if self.cube.len() > other.cube.len() {
            return (false, None);
        }
        let ss = self.var_sign();
        if ss & other.var_sign() != ss {
            return (false, None);
        }
        self.cube.ordered_subsume_execpt_one(&other.cube)
    }

    #[inline]
    pub fn subsume_set(&self, other: &Lemma, other_lits: &LitSet) -> bool {
        if self.cube.len() > other.cube.len() {
            return false;
        }
        if self.sign & other.sign != self.sign {
            return false;
        }
        for l in self.iter() {
            if !other_lits.has(*l) {
                return false;
            }
        }
        true
    }
}

impl Hash for Lemma {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl Display for Lemma {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.cube, f)
    }
}

pub fn cnf_lits_and(master: Lit, lits: &[Lit]) -> Vec<LitVec> {
    let mut cnf = Vec::new();
    let mut cls = LitVec::from([master]);
    for l in lits.iter() {
        cnf.push(LitVec::from([!master, *l]));
        cls.push(!*l);
    }
    cnf.push(cls);
    cnf
}

pub fn cnf_lits_or(master: Lit, lits: &[Lit]) -> Vec<LitVec> {
    let mut cnf = Vec::new();
    let mut cls = LitVec::from([!master]);
    for l in lits.iter() {
        cnf.push(LitVec::from([master, !*l]));
        cls.push(*l);
    }
    cnf.push(cls);
    cnf
}
