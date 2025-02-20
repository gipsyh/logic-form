pub mod simplify;

use crate::{Lit, LitVec, LitVvec, Var, VarMap};
use giputils::hash::{GHashMap, GHashSet};
use simplify::DagCnfSimplify;
use std::{
    iter::{once, Flatten},
    slice,
};

#[derive(Debug, Clone)]
pub struct DagCnf {
    max_var: Var,
    cnf: VarMap<LitVvec>,
    pub dep: VarMap<Vec<Var>>,
}

impl DagCnf {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn new_var(&mut self) -> Var {
        self.max_var += 1;
        self.dep.reserve(self.max_var);
        self.cnf.reserve(self.max_var);
        self.max_var
    }

    #[inline]
    pub fn new_var_to(&mut self, n: Var) {
        while self.max_var < n {
            self.new_var();
        }
    }

    #[inline]
    pub fn max_var(&self) -> Var {
        self.max_var
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    #[inline]
    pub fn iter(&self) -> Flatten<slice::Iter<'_, LitVvec>> {
        self.cnf.iter().flatten()
    }

    #[inline]
    pub fn add_rel(&mut self, n: Var, rel: &[LitVec]) {
        assert!(self.dep[n].is_empty() && self.cnf[n].is_empty());
        let mut dep = GHashSet::from_iter(rel.iter().flatten().map(|l| l.var()));
        dep.remove(&n);
        self.dep[n].extend(dep.iter());
        self.cnf[n].extend_from_slice(rel);
    }

    pub fn get_coi(&self, var: impl Iterator<Item = Var>) -> GHashSet<Var> {
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
        marked
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

    pub fn arrange(&mut self, additional: impl Iterator<Item = Var>) -> GHashMap<Var, Var> {
        let mut domain = GHashSet::from_iter(additional.chain(once(Var::CONST)));
        for cls in self.iter() {
            for l in cls.iter() {
                domain.insert(l.var());
            }
        }
        let mut domain = Vec::from_iter(domain);
        domain.sort();
        let mut domain_map = GHashMap::new();
        let mut res = DagCnf::new();
        for (i, d) in domain.iter().enumerate() {
            let v = Var::new(i);
            res.new_var_to(v);
            domain_map.insert(*d, v);
        }
        let map_lit = |l: &Lit| Lit::new(domain_map[&l.var()], l.polarity());
        for (d, v) in domain_map.iter() {
            if d.is_constant() {
                continue;
            }
            let mut new_cls = Vec::new();
            for cls in self.cnf[*d].iter() {
                new_cls.push(cls.iter().map(map_lit).collect());
            }
            res.add_rel(*v, &new_cls);
        }
        *self = res;
        domain_map
    }

    pub fn simplify(&self, frozen: impl Iterator<Item = Var>) -> Self {
        let mut simp = DagCnfSimplify::new(self);
        for v in frozen {
            simp.froze(v);
        }
        simp.simplify()
    }
}

impl Default for DagCnf {
    fn default() -> Self {
        let max_var = Var::CONST;
        let mut cnf: VarMap<LitVvec> = VarMap::new_with(max_var);
        cnf[max_var].push(LitVec::from(Lit::constant(true)));
        Self {
            max_var,
            cnf,
            dep: VarMap::new_with(max_var),
        }
    }
}
