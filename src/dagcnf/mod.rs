use crate::{Lit, LitVec, LitVvec, Var, VarMap};
use giputils::hash::{GHashMap, GHashSet};

#[derive(Debug, Clone)]
pub struct DagCnf {
    max_var: Var,
    cnf: VarMap<LitVvec>,
    pub dep: VarMap<Vec<Var>>,
}

impl DagCnf {
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
    pub fn max_var(&self) -> Var {
        self.max_var
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

    pub fn arrange(&mut self, additional: impl Iterator<Item = Var>) -> GHashMap<Var, Var> {
        let root = Vec::from_iter(self.root());
        let map = self.cnf.arrange(additional);
        let mut compressed = GHashSet::new();
        for v in root {
            self.compress_deps(v, &map, &mut compressed);
        }
        let mut dep = VarMap::new_with(self.max_var);
        for (f, t) in map.iter() {
            let fdep: Vec<Var> = self.dep[*f].iter().map(|v| map[v]).collect();
            dep[*t] = fdep;
        }
        self.dep = dep;
        map
    }

    /// # Safety
    pub unsafe fn set_cls(&mut self, cls: Vec<LitVec>) {
        todo!()
    }

    // pub fn simplify(&self, frozen: impl Iterator<Item = Var>) -> Cnf {
    //     // let mut simp = CnfSimplify::new(self.cnf.clone());
    //     // for v in frozen {
    //     //     simp.froze(v);
    //     // }
    //     // simp.simplify()
    //     todo!()
    // }
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
