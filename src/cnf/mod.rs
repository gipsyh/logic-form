pub mod simp;

use crate::{Lit, LitVec, Var, VarMap};
use giputils::hash::{GHashMap, GHashSet};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Cnf {
    max_var: Var,
    cls: Vec<LitVec>,
}

impl Cnf {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn max_var(&self) -> Var {
        self.max_var
    }

    #[inline]
    pub fn new_var(&mut self) -> Var {
        self.max_var += 1;
        self.max_var
    }

    #[inline]
    pub fn new_var_to(&mut self, n: Var) {
        self.max_var = self.max_var.max(n);
    }

    #[inline]
    pub fn add_clause(&mut self, cls: &[Lit]) {
        self.cls.push(LitVec::from(cls));
    }

    #[inline]
    pub fn add_clauses(&mut self, cls: impl Iterator<Item = LitVec>) {
        self.cls.extend(cls);
    }

    #[inline]
    pub fn clauses(&self) -> &[LitVec] {
        &self.cls
    }

    #[inline]
    pub fn add_assign_rel(&mut self, n: Lit, s: Lit) {
        let rel = vec![LitVec::from([n, !s]), LitVec::from([!n, s])];
        self.add_clauses(rel.into_iter());
    }

    #[inline]
    pub fn add_and_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        let rel = vec![
            LitVec::from([x, !n]),
            LitVec::from([y, !n]),
            LitVec::from([!x, !y, n]),
        ];
        self.add_clauses(rel.into_iter());
    }

    #[inline]
    pub fn add_or_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        let rel = vec![
            LitVec::from([!x, n]),
            LitVec::from([!y, n]),
            LitVec::from([x, y, !n]),
        ];
        self.add_clauses(rel.into_iter());
    }

    #[inline]
    pub fn add_xor_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        let rel = vec![
            LitVec::from([!x, y, n]),
            LitVec::from([x, !y, n]),
            LitVec::from([x, y, !n]),
            LitVec::from([!x, !y, !n]),
        ];
        self.add_clauses(rel.into_iter());
    }

    #[inline]
    pub fn add_xnor_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        let rel = vec![
            LitVec::from([!x, y, !n]),
            LitVec::from([x, !y, !n]),
            LitVec::from([x, y, n]),
            LitVec::from([!x, !y, n]),
        ];
        self.add_clauses(rel.into_iter());
    }

    #[inline]
    pub fn add_ite_rel(&mut self, n: Lit, c: Lit, t: Lit, e: Lit) {
        let rel = vec![
            LitVec::from([t, !c, !n]),
            LitVec::from([!t, !c, n]),
            LitVec::from([e, c, !n]),
            LitVec::from([!e, c, n]),
        ];
        self.add_clauses(rel.into_iter());
    }

    pub fn arrange(&mut self) -> GHashMap<Var, Var> {
        let mut domain = GHashSet::new();
        domain.insert(Var::new(0));
        for cls in self.cls.iter() {
            for l in cls.iter() {
                domain.insert(l.var());
            }
        }
        let mut domain = Vec::from_iter(domain);
        domain.sort();
        let mut domain_map = GHashMap::new();
        for (i, d) in domain.iter().enumerate() {
            domain_map.insert(*d, Var::new(i));
        }
        let map_lit = |l: &Lit| Lit::new(domain_map[&l.var()], l.polarity());
        for cls in self.cls.iter_mut() {
            for l in cls.iter_mut() {
                *l = map_lit(l);
            }
        }
        self.max_var = Var::new(domain.len() - 1);
        domain_map
    }

    /// # Safety
    pub unsafe fn set_cls(&mut self, cls: Vec<LitVec>) {
        self.cls = cls;
    }
}

impl Deref for Cnf {
    type Target = [LitVec];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.cls
    }
}

impl DerefMut for Cnf {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cls
    }
}

impl Default for Cnf {
    fn default() -> Self {
        Self {
            max_var: Var(0),
            cls: vec![LitVec::from([Lit::constant(true)])],
        }
    }
}

#[derive(Debug, Clone)]
pub struct DagCnf {
    cnf: Cnf,
    pub dep: VarMap<Vec<Var>>,
}

impl DagCnf {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn new_var(&mut self) -> Var {
        let n = self.cnf.new_var();
        self.dep.reserve(n);
        n
    }

    #[inline]
    pub fn max_var(&self) -> Var {
        self.cnf.max_var()
    }

    #[inline]
    pub fn add_rel(&mut self, n: Var, rel: &[LitVec]) {
        let mut dep = GHashSet::from_iter(rel.iter().flatten().map(|l| l.var()));
        dep.remove(&n);
        self.dep[n].extend(dep.iter());
    }

    #[inline]
    pub fn add_assign_rel(&mut self, n: Lit, s: Lit) {
        self.cnf.add_assign_rel(n, s);
        self.dep[n.var()].push(s.var());
    }

    #[inline]
    pub fn add_and_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        self.cnf.add_and_rel(n, x, y);
        self.dep[n.var()].extend_from_slice(&[x.var(), y.var()]);
    }

    #[inline]
    pub fn add_or_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        self.cnf.add_or_rel(n, x, y);
        self.dep[n.var()].extend_from_slice(&[x.var(), y.var()]);
    }

    #[inline]
    pub fn add_xor_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        self.cnf.add_xor_rel(n, x, y);
        self.dep[n.var()].extend_from_slice(&[x.var(), y.var()]);
    }

    #[inline]
    pub fn add_xnor_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        self.cnf.add_xnor_rel(n, x, y);
        self.dep[n.var()].extend_from_slice(&[x.var(), y.var()]);
    }

    #[inline]
    pub fn add_ite_rel(&mut self, n: Lit, c: Lit, t: Lit, e: Lit) {
        self.cnf.add_ite_rel(n, c, t, e);
        self.dep[n.var()].extend_from_slice(&[c.var(), t.var(), e.var()]);
    }

    pub fn get_coi(&self, var: impl Iterator<Item = Var>) -> Vec<Var> {
        let mut marked = GHashSet::new();
        let mut queue = vec![];
        for v in var {
            marked.insert(v);
            queue.push(v);
        }
        while let Some(v) = queue.pop() {
            for d in self.dep[v].iter() {
                if !marked.contains(d) {
                    marked.insert(*d);
                    queue.push(*d);
                }
            }
        }
        Vec::from_iter(marked)
    }

    pub fn root(&self) -> GHashSet<Var> {
        let mut root = GHashSet::from_iter(
            (Var::new(0)..=self.max_var()).filter(|v| !self.dep[*v].is_empty()),
        );
        for d in self.dep.iter() {
            for d in d.iter() {
                root.remove(d);
            }
        }
        root
    }

    fn compress_deps(
        &mut self,
        v: Var,
        domain: &GHashMap<Var, Var>,
        compressed: &mut GHashSet<Var>,
    ) {
        if compressed.contains(&v) {
            return;
        }
        for d in self.dep[v].clone() {
            self.compress_deps(d, domain, compressed);
        }
        let mut dep = GHashSet::new();
        for d in self.dep[v].iter() {
            if domain.contains_key(d) {
                dep.insert(*d);
                continue;
            }
            for dd in self.dep[*d].iter() {
                dep.insert(*dd);
            }
        }
        self.dep[v] = dep.into_iter().collect();
        compressed.insert(v);
    }

    pub fn arrange(&mut self) -> GHashMap<Var, Var> {
        let root = Vec::from_iter(self.root());
        let map = self.cnf.arrange();
        let mut compressed = GHashSet::new();
        for v in root {
            self.compress_deps(v, &map, &mut compressed);
        }
        let mut dep = VarMap::new_with(self.cnf.max_var());
        for (f, t) in map.iter() {
            let fdep: Vec<Var> = self.dep[*f].iter().map(|v| map[v]).collect();
            dep[*t] = fdep;
        }
        self.dep = dep;
        map
    }

    /// # Safety
    pub unsafe fn set_cls(&mut self, cls: Vec<LitVec>) {
        self.cnf.set_cls(cls);
    }
}

impl Default for DagCnf {
    fn default() -> Self {
        Self {
            cnf: Default::default(),
            dep: VarMap::new_with(Var(0)),
        }
    }
}

impl Deref for DagCnf {
    type Target = [LitVec];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.cnf.deref()
    }
}
