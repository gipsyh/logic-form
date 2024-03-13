use crate::{Lit, Var};
use std::{
    ops::{Deref, DerefMut, Index, IndexMut},
    slice,
};

#[derive(Debug, Default, Clone)]
pub struct VarMap<T> {
    map: Vec<T>,
}

impl<T: Default> VarMap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn reserve(&mut self, var: Var) {
        let len = Into::<usize>::into(var) + 1;
        if self.len() < len {
            self.map.resize_with(len, Default::default)
        }
    }
}

impl<T> Index<Var> for VarMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Var) -> &Self::Output {
        #[cfg(feature = "no_bound_check")]
        unsafe {
            self.map.get_unchecked(index.0 as usize)
        }
        #[cfg(not(feature = "no_bound_check"))]
        &self.map[index.0 as usize]
    }
}

impl<T> IndexMut<Var> for VarMap<T> {
    #[inline]
    fn index_mut(&mut self, index: Var) -> &mut Self::Output {
        #[cfg(feature = "no_bound_check")]
        unsafe {
            self.map.get_unchecked_mut(index.0 as usize)
        }
        #[cfg(not(feature = "no_bound_check"))]
        &mut self.map[index.0 as usize]
    }
}

impl<T> Index<Lit> for VarMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Lit) -> &Self::Output {
        #[cfg(feature = "no_bound_check")]
        unsafe {
            self.map.get_unchecked((index.0 >> 1) as usize)
        }
        #[cfg(not(feature = "no_bound_check"))]
        &self.map[(index.0 >> 1) as usize]
    }
}

impl<T> IndexMut<Lit> for VarMap<T> {
    #[inline]
    fn index_mut(&mut self, index: Lit) -> &mut Self::Output {
        #[cfg(feature = "no_bound_check")]
        unsafe {
            self.map.get_unchecked_mut((index.0 >> 1) as usize)
        }
        #[cfg(not(feature = "no_bound_check"))]
        &mut self.map[(index.0 >> 1) as usize]
    }
}

impl<T> Deref for VarMap<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T> DerefMut for VarMap<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Debug, Default, Clone)]
pub struct LitMap<T> {
    map: Vec<T>,
}

impl<T: Default> LitMap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn reserve(&mut self, var: Var) {
        let len = (Into::<usize>::into(var) + 1) * 2;
        if self.len() < len {
            self.map.resize_with(len, Default::default)
        }
    }
}

impl<T> Index<Lit> for LitMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Lit) -> &Self::Output {
        #[cfg(feature = "no_bound_check")]
        unsafe {
            self.map.get_unchecked(index.0 as usize)
        }
        #[cfg(not(feature = "no_bound_check"))]
        &self.map[index.0 as usize]
    }
}

impl<T> IndexMut<Lit> for LitMap<T> {
    #[inline]
    fn index_mut(&mut self, index: Lit) -> &mut Self::Output {
        #[cfg(feature = "no_bound_check")]
        unsafe {
            self.map.get_unchecked_mut(index.0 as usize)
        }
        #[cfg(not(feature = "no_bound_check"))]
        &mut self.map[index.0 as usize]
    }
}

impl<T> Deref for LitMap<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T> DerefMut for LitMap<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Default)]
pub struct VarSet {
    set: Vec<Var>,
    has: VarMap<bool>,
}

impl VarSet {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.set.len() as _
    }

    #[inline]
    pub fn reserve(&mut self, var: Var) {
        self.has.reserve(var);
    }

    #[inline]
    pub fn has(&self, var: Var) -> bool {
        self.has[var]
    }

    #[inline]
    pub fn insert(&mut self, var: Var) {
        if !self.has[var] {
            self.set.push(var);
            self.has[var] = true;
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        for l in self.set.iter() {
            self.has[*l] = false;
        }
        self.set.clear();
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<Var> {
        self.set.iter()
    }
}

impl Index<u32> for VarSet {
    type Output = Var;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        &self.set[index as usize]
    }
}

#[derive(Default, Debug)]
pub struct LitSet {
    pub set: Vec<Lit>,
    has: LitMap<bool>,
}

impl LitSet {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn reserve(&mut self, var: Var) {
        self.has.reserve(var);
    }

    #[inline]
    pub fn insert(&mut self, lit: Lit) {
        if !self.has[lit] {
            self.set.push(lit);
            self.has[lit] = true;
        }
    }

    #[inline]
    pub fn has(&self, lit: Lit) -> bool {
        self.has[lit]
    }

    #[inline]
    pub fn clear(&mut self) {
        for l in self.set.iter() {
            self.has[*l] = false;
        }
        self.set.clear();
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<Lit> {
        self.set.iter()
    }
}

#[derive(Default)]
pub struct VarRef {
    set: Vec<Var>,
    refs: VarMap<u32>,
    dirty: VarMap<bool>,
}

impl VarRef {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn reserve(&mut self, var: Var) {
        self.refs.reserve(var);
        self.dirty.reserve(var);
    }

    pub fn inref(&mut self, var: Var) {
        self.refs[var] += 1;
        if self.refs[var] != 0 {
            if self.dirty[var] {
                self.dirty[var] = false
            } else {
                self.set.push(var)
            }
        }
    }

    #[inline]
    pub fn deref(&mut self, var: Var) {
        assert!(self.refs[var] > 0);
        self.refs[var] -= 1;
        if self.refs[var] == 0 {
            self.dirty[var] = true;
        }
    }

    #[inline]
    pub fn iter(&self) -> VarRefIter {
        VarRefIter {
            varref: self as *const VarRef as *mut VarRef,
            p: 0,
        }
    }
}

pub struct VarRefIter {
    varref: *mut VarRef,
    p: usize,
}

impl Iterator for VarRefIter {
    type Item = Var;

    fn next(&mut self) -> Option<Self::Item> {
        let varref = unsafe { &mut *self.varref };
        while self.p < varref.set.len() && varref.dirty[varref.set[self.p]] {
            varref.dirty[varref.set[self.p]] = false;
            varref.set.swap_remove(self.p);
        }
        if self.p >= varref.set.len() {
            return None;
        }
        self.p += 1;
        Some(varref.set[self.p - 1])
    }
}
