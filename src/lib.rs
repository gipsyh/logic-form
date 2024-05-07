#![feature(is_sorted)]

mod dimacs;
pub mod fol;
mod utils;

pub use utils::*;

use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Debug, Display},
    ops::{Add, Deref, DerefMut, Not},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Var(u32);

impl Var {
    #[inline]
    pub fn new(x: usize) -> Self {
        Self(x as _)
    }

    #[inline]
    pub fn lit(&self) -> Lit {
        Lit(self.0 << 1)
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord, Default)]
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
        val.0 as i32
    }
}

impl From<Lit> for usize {
    #[inline]
    fn from(val: Lit) -> Self {
        val.0 as usize
    }
}

impl From<i32> for Lit {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value as u32)
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
    pub fn constant_lit(polarity: bool) -> Self {
        Self::new(Var::new(0), !polarity)
    }

    #[inline]
    pub fn is_constant(&self, polarity: bool) -> bool {
        *self == Self::constant_lit(polarity)
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.polarity() {
            write!(f, "{}", self.var())
        } else {
            write!(f, "-{}", self.var())
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Clause {
    lits: Vec<Lit>,
}

impl Clause {
    pub fn new() -> Self {
        Clause { lits: Vec::new() }
    }
}

impl Default for Clause {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Clause {
    type Target = Vec<Lit>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lits
    }
}

impl DerefMut for Clause {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lits
    }
}

impl Not for Clause {
    type Output = Cube;

    #[inline]
    fn not(self) -> Self::Output {
        let lits = self.lits.iter().map(|lit| !*lit).collect();
        Cube { lits }
    }
}

impl Not for &Clause {
    type Output = Cube;

    #[inline]
    fn not(self) -> Self::Output {
        let lits = self.lits.iter().map(|lit| !*lit).collect();
        Cube { lits }
    }
}

impl<F: Into<Vec<Lit>>> From<F> for Clause {
    fn from(value: F) -> Self {
        Self { lits: value.into() }
    }
}

impl FromIterator<Lit> for Clause {
    fn from_iter<T: IntoIterator<Item = Lit>>(iter: T) -> Self {
        Self {
            lits: Vec::from_iter(iter),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cube {
    lits: Vec<Lit>,
}

impl Cube {
    #[inline]
    pub fn new() -> Self {
        Cube { lits: Vec::new() }
    }

    #[inline]
    pub fn subsume(&self, cube: &Cube) -> bool {
        let x_lit_set = self.iter().collect::<HashSet<&Lit>>();
        let y_lit_set = cube.iter().collect::<HashSet<&Lit>>();
        x_lit_set.is_subset(&y_lit_set)
    }

    #[inline]
    pub fn ordered_subsume(&self, cube: &Cube) -> bool {
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
    pub fn intersection(&self, cube: &Cube) -> Cube {
        let x_lit_set = self.iter().collect::<HashSet<&Lit>>();
        let y_lit_set = cube.iter().collect::<HashSet<&Lit>>();
        Self {
            lits: x_lit_set
                .intersection(&y_lit_set)
                .copied()
                .copied()
                .collect(),
        }
    }

    #[inline]
    pub fn ordered_intersection(&self, cube: &Cube) -> Cube {
        debug_assert!(self.is_sorted_by_key(|l| l.var()));
        debug_assert!(cube.is_sorted_by_key(|l| l.var()));
        let mut res = Cube::new();
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
}

impl Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Cube {
    type Target = Vec<Lit>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lits
    }
}

impl DerefMut for Cube {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lits
    }
}

impl PartialOrd for Cube {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cube {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // debug_assert!(self.is_sorted_by_key(|x| x.var()));
        // debug_assert!(other.is_sorted_by_key(|x| x.var()));
        let min_index = self.len().min(other.len());
        for i in 0..min_index {
            match self.lits[i].0.cmp(&other.lits[i].0) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
                Ordering::Greater => return Ordering::Greater,
            }
        }
        self.len().cmp(&other.len())
    }
}

impl Not for Cube {
    type Output = Clause;

    #[inline]
    fn not(self) -> Self::Output {
        let lits = self.lits.iter().map(|lit| !*lit).collect();
        Clause { lits }
    }
}

impl Not for &Cube {
    type Output = Clause;

    #[inline]
    fn not(self) -> Self::Output {
        let lits = self.lits.iter().map(|lit| !*lit).collect();
        Clause { lits }
    }
}

impl<const N: usize> From<[Lit; N]> for Cube {
    #[inline]
    fn from(s: [Lit; N]) -> Self {
        Self { lits: Vec::from(s) }
    }
}

impl From<&[Lit]> for Cube {
    #[inline]
    fn from(s: &[Lit]) -> Self {
        Self { lits: Vec::from(s) }
    }
}

impl From<Cube> for Vec<Lit> {
    #[inline]
    fn from(val: Cube) -> Self {
        val.lits
    }
}

impl FromIterator<Lit> for Cube {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Lit>>(iter: T) -> Self {
        Self {
            lits: Vec::from_iter(iter),
        }
    }
}

impl IntoIterator for Cube {
    type Item = Lit;

    type IntoIter = std::vec::IntoIter<Lit>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.lits.into_iter()
    }
}

#[derive(Clone, Debug)]
pub struct Cnf {
    clauses: Vec<Clause>,
}

impl Cnf {
    pub fn new() -> Self {
        Self {
            clauses: Vec::new(),
        }
    }

    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
}

impl Default for Cnf {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Cnf {
    type Target = Vec<Clause>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.clauses
    }
}

impl DerefMut for Cnf {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clauses
    }
}

impl<F: Into<Vec<Clause>>> From<F> for Cnf {
    fn from(value: F) -> Self {
        Self {
            clauses: value.into(),
        }
    }
}

impl FromIterator<Clause> for Cnf {
    fn from_iter<T: IntoIterator<Item = Clause>>(iter: T) -> Self {
        Self {
            clauses: Vec::from_iter(iter),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dnf {
    cubes: Vec<Cube>,
}

impl Dnf {
    pub fn new() -> Self {
        Self { cubes: Vec::new() }
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.push(cube);
    }
}

impl Default for Dnf {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Dnf {
    type Target = Vec<Cube>;

    fn deref(&self) -> &Self::Target {
        &self.cubes
    }
}

impl DerefMut for Dnf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cubes
    }
}

impl FromIterator<Cube> for Dnf {
    fn from_iter<T: IntoIterator<Item = Cube>>(iter: T) -> Self {
        Self {
            cubes: Vec::from_iter(iter),
        }
    }
}

impl Add for Dnf {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.cubes.append(&mut rhs.cubes);
        self
    }
}

impl Not for Dnf {
    type Output = Cnf;

    fn not(self) -> Self::Output {
        let mut cnf = Cnf::new();
        for cube in self.cubes {
            cnf.add_clause(!cube);
        }
        cnf
    }
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct Lemma {
    cube: Cube,
    sign: u64,
}

impl Deref for Lemma {
    type Target = Cube;

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
        if self.sign != other.sign || self.len() != other.len() {
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

impl Lemma {
    #[inline]
    pub fn new(mut cube: Cube) -> Self {
        cube.sort();
        let mut sign = 0;
        for l in cube.iter() {
            sign |= 1 << (Into::<u32>::into(*l) % 63);
        }
        Self { cube, sign }
    }

    #[inline]
    pub fn cube(&self) -> &Cube {
        &self.cube
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
