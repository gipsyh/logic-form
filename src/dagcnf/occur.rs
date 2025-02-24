use crate::{Lit, LitMap, Var};
use giputils::{grc::Grc, gvec::Gvec, hash::GHashSet};

#[derive(Debug, Clone, Default)]
pub struct Occur {
    occur: Gvec<u32>,
    dirty: bool,
    size: usize,
}

impl Occur {
    #[inline]
    fn len(&self) -> usize {
        self.size
    }

    #[inline]
    #[allow(unused)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn clean(&mut self, removed: &GHashSet<u32>) {
        if self.dirty {
            self.occur.retain(|&i| !removed.contains(&i));
            self.dirty = false;
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.occur.clear();
        self.dirty = false;
        self.size = 0;
    }

    #[inline]
    pub fn add(&mut self, c: u32) {
        self.occur.push(c);
        self.size += 1;
    }

    #[inline]
    pub fn lazy_remove(&mut self) {
        self.dirty = true;
        self.size -= 1;
    }

    // pub fn remove(&mut self, cls: u32) {
    //     self.clean();
    //     self.size -= 1;
    //     let len = self.occur.len();
    //     self.occur.retain(|&i| i != cls);
    //     assert!(len == self.occur.len() + 1);
    // }
}

pub struct Occurs {
    occurs: LitMap<Occur>,
    removed: Grc<GHashSet<u32>>,
}

impl Occurs {
    #[inline]
    #[allow(unused)]
    pub fn new(removed: Grc<GHashSet<u32>>) -> Self {
        Self {
            occurs: LitMap::new(),
            removed,
        }
    }

    #[inline]
    pub fn new_with(var: Var, removed: Grc<GHashSet<u32>>) -> Self {
        Self {
            occurs: LitMap::new_with(var),
            removed,
        }
    }

    #[inline]
    #[allow(unused)]
    pub fn reserve(&mut self, var: Var) {
        self.occurs.reserve(var);
    }

    #[inline]
    pub fn num_occur(&self, l: Lit) -> usize {
        self.occurs[l].len()
    }

    #[inline]
    pub fn add(&mut self, lit: Lit, o: u32) {
        self.occurs[lit].add(o);
    }

    #[inline]
    pub fn del(&mut self, lit: Lit, _o: u32) {
        self.occurs[lit].lazy_remove();
    }

    #[inline]
    pub fn get(&mut self, lit: Lit) -> &[u32] {
        self.occurs[lit].clean(&self.removed);
        &self.occurs[lit].occur
    }
}
