use super::Cnf;
use crate::{Lit, LitMap, Var};
use giputils::hash::GHashSet;

pub struct CnfSimplify {
    cnf: Cnf,
    qassign: Vec<Lit>,
    qassign_head: usize,
    occurs: LitMap<Vec<usize>>,
    removed: GHashSet<usize>,
    frozen: GHashSet<Var>,
}

impl CnfSimplify {
    pub fn new(cnf: Cnf) -> Self {
        let mut occurs: LitMap<Vec<_>> = LitMap::new_with(cnf.max_var());
        for (i, cls) in cnf.cls.iter().enumerate() {
            for &lit in cls.iter() {
                occurs[lit].push(i);
            }
        }
        Self {
            cnf,
            qassign: Vec::new(),
            qassign_head: 0,
            occurs,
            removed: GHashSet::new(),
            frozen: GHashSet::new(),
        }
    }

    pub fn froze(&mut self, v: Var) {
        self.frozen.insert(v);
    }

    fn add_clause(&mut self, cls: &[Lit]) {
        assert!(cls.len() > 1);
        let i = self.cnf.len();
        self.cnf.add_clause(cls);
        for &lit in self.cnf[i].iter() {
            self.occurs[lit].push(i);
        }
    }

    fn remove_cls(&mut self, i: usize) {
        for &lit in self.cnf[i].iter() {
            self.occurs[lit].retain(|&j| j != i);
        }
        self.removed.insert(i);
    }

    fn remove_lit(&mut self, lit: Lit) {
        for s in self.occurs[lit].clone() {
            self.cnf[s].retain(|&l| l != lit);
            assert!(!self.cnf[s].is_empty());
            if self.cnf[s].len() == 1 {
                self.qassign.push(self.cnf[s][0]);
            }
        }
        self.occurs[lit].clear();
    }

    fn const_prop(&mut self) {
        while self.qassign_head < self.qassign.len() {
            let lit = self.qassign[self.qassign_head];
            for s in self.occurs[lit].clone() {
                self.remove_cls(s);
            }
            self.remove_lit(!lit);
            self.qassign_head += 1;
        }
    }

    fn const_simp(&mut self) {
        let mut qassign = Vec::new();
        for cls in self.cnf.iter() {
            if cls.len() == 1 {
                qassign.push(cls[0]);
            }
        }
        self.const_prop();
    }

    fn eliminate(&mut self, v: Var) {
        if self.frozen.contains(&v) {
            return;
        }
        let l = v.lit();
        let origin_cost = self.occurs[l].len() + self.occurs[!l].len();
        let mut new_cnf = Vec::new();
        for x in self.occurs[l].iter() {
            for y in self.occurs[!l].iter() {
                if let Some(r) = self.cnf[*x].resolvent(&self.cnf[*y], v) {
                    new_cnf.push(r);
                }
            }
        }
        for n in new_cnf.iter() {
            assert!(!n.is_empty());
            if n.len() == 1 {
                todo!();
            }
        }
        if new_cnf.len() < origin_cost {
            for cls in Vec::from_iter(self.occurs[l].iter().chain(self.occurs[!l].iter()).cloned())
            {
                self.remove_cls(cls);
            }
        }
        for n in new_cnf {
            self.add_clause(&n);
        }
    }

    fn bve_simp(&mut self) {
        for v in Var::new(1)..=self.cnf.max_var() {
            self.eliminate(v);
        }
        dbg!(self.cnf.len() - self.removed.len());
    }

    pub fn simplify(&mut self) {
        self.const_simp();
    }
}
