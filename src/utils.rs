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

    pub fn reserve(&mut self, var: Var) {
        self.map
            .resize_with(Into::<usize>::into(var) + 1, Default::default)
    }
}

impl<T> Index<Var> for VarMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Var) -> &Self::Output {
        // &self.map[index.0 as usize]
        unsafe { self.map.get_unchecked(index.0 as usize) }
    }
}

impl<T> IndexMut<Var> for VarMap<T> {
    #[inline]
    fn index_mut(&mut self, index: Var) -> &mut Self::Output {
        // &mut self.map[index.0 as usize]
        unsafe { self.map.get_unchecked_mut(index.0 as usize) }
    }
}

impl<T> Index<Lit> for VarMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Lit) -> &Self::Output {
        // &self.map[(index.0 >> 1) as usize]
        unsafe { self.map.get_unchecked((index.0 >> 1) as usize) }
    }
}

impl<T> IndexMut<Lit> for VarMap<T> {
    #[inline]
    fn index_mut(&mut self, index: Lit) -> &mut Self::Output {
        // &mut self.map[(index.0 >> 1) as usize]
        unsafe { self.map.get_unchecked_mut((index.0 >> 1) as usize) }
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

#[derive(Debug, Default)]
pub struct LitMap<T> {
    map: Vec<T>,
}

impl<T: Default> LitMap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn reserve(&mut self, var: Var) {
        self.map
            .resize_with((Into::<usize>::into(var) + 1) * 2, Default::default)
    }
}

impl<T> Index<Lit> for LitMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Lit) -> &Self::Output {
        // &self.map[index.0 as usize]
        unsafe { self.map.get_unchecked(index.0 as usize) }
    }
}

impl<T> IndexMut<Lit> for LitMap<T> {
    #[inline]
    fn index_mut(&mut self, index: Lit) -> &mut Self::Output {
        // &mut self.map[index.0 as usize]
        unsafe { self.map.get_unchecked_mut(index.0 as usize) }
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

#[derive(Default, Debug)]
pub struct LitSet {
    set: Vec<Lit>,
    has: LitMap<bool>,
}

impl LitSet {
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
