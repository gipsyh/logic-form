use crate::{Lit, LitVec, Var, VarMap};
use std::ops::Deref;

#[derive(Debug)]
pub struct Cnf {
    max_var: Var,
    pub cls: Vec<LitVec>,
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
}

impl Deref for Cnf {
    type Target = [LitVec];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.cls
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

#[derive(Debug)]
pub struct DagCnf {
    pub cnf: Cnf,
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
