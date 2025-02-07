use super::Cnf;
use crate::{Clause, Cube, Lemma, Lit, LitMap, Var};
use giputils::hash::GHashSet;
use std::time::Instant;

#[derive(Debug, Clone, Default)]
struct Occurs {
    occurs: Vec<usize>,
    dirty: bool,
    size: usize,
}

impl Occurs {
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
    fn clean(&mut self, removed: &GHashSet<usize>) {
        if self.dirty {
            self.occurs.retain(|&i| !removed.contains(&i));
            self.dirty = false;
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.occurs.clear();
        self.dirty = false;
        self.size = 0;
    }

    #[inline]
    fn add(&mut self, c: usize) {
        self.occurs.push(c);
        self.size += 1;
    }

    #[inline]
    fn remove(&mut self) {
        self.dirty = true;
        self.size -= 1;
    }
}

pub struct CnfSimplify {
    cnf: Cnf,
    qassign: Vec<Lit>,
    qassign_head: usize,
    occurs: LitMap<Occurs>,
    removed: GHashSet<usize>,
    frozen: GHashSet<Var>,
}

impl CnfSimplify {
    pub fn new(cnf: Cnf) -> Self {
        let mut occurs: LitMap<Occurs> = LitMap::new_with(cnf.max_var());
        for (i, cls) in cnf.cls.iter().enumerate() {
            for &lit in cls.iter() {
                occurs[lit].add(i);
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

    #[inline]
    fn get_occurs(&mut self, lit: Lit) -> Vec<usize> {
        if self.occurs[lit].dirty {
            self.occurs[lit].clean(&self.removed);
        }
        self.occurs[lit].occurs.clone()
    }

    pub fn froze(&mut self, v: Var) {
        self.frozen.insert(v);
    }

    fn add_clause(&mut self, cls: &[Lit]) {
        assert!(cls.len() > 1);
        let i = self.cnf.len();
        self.cnf.add_clause(cls);
        for &lit in self.cnf[i].iter() {
            self.occurs[lit].add(i);
        }
    }

    fn remove_cls(&mut self, i: usize) {
        for &lit in self.cnf[i].iter() {
            self.occurs[lit].remove();
        }
        self.removed.insert(i);
    }

    fn remove_lit(&mut self, lit: Lit) {
        for s in self.get_occurs(lit) {
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
            for s in self.get_occurs(lit).clone() {
                self.remove_cls(s);
            }
            self.remove_lit(!lit);
            self.qassign_head += 1;
        }
    }

    fn const_simp(&mut self) {
        for cls in self.cnf.iter() {
            if cls.len() == 1 {
                self.qassign.push(cls[0]);
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
        let pos = self.get_occurs(l);
        let neg = self.get_occurs(!l);
        let mut new_cnf = Vec::new();
        for x in pos.iter() {
            for y in neg.iter() {
                if let Some(r) = self.cnf[*x].resolvent(&self.cnf[*y], v) {
                    if r.len() > 20 {
                        return;
                    }
                    new_cnf.push(r);
                }
                if new_cnf.len() > origin_cost {
                    return;
                }
            }
        }
        for n in new_cnf.iter() {
            assert!(!n.is_empty());
            if n.len() == 1 {
                todo!();
            }
        }
        for cls in pos.into_iter().chain(neg) {
            self.remove_cls(cls);
        }
        let new_cnf = clause_subsume_simplify(new_cnf);
        for n in new_cnf {
            self.add_clause(&n);
        }
    }

    fn bve_simp(&mut self) {
        let mut v = Vec::from_iter(Var::CONST..=self.cnf.max_var());
        v.sort_by_key(|v| self.occurs[v.lit()].len() + self.occurs[!v.lit()].len());
        for v in v.iter() {
            self.eliminate(*v);
        }
    }

    pub fn simplify(&mut self) -> Cnf {
        self.const_simp();
        let start = Instant::now();
        self.bve_simp();
        dbg!(start.elapsed());
        let mut res = Cnf::new();
        for s in 0..self.cnf.len() {
            if !self.removed.contains(&s) {
                res.add_clause(&self.cnf[s]);
            }
        }
        for a in self.qassign.iter() {
            if !a.var().is_constant() && self.frozen.contains(&a.var()) {
                res.add_clause(&[*a]);
            }
        }
        res
    }
}

fn clause_subsume_simplify(lemmas: Vec<Clause>) -> Vec<Clause> {
    let mut lemmas: Vec<Lemma> = lemmas
        .iter()
        .map(|cls| Lemma::new(Cube::from(cls.as_slice())))
        .collect();
    lemmas.sort_by_key(|l| l.len());
    let mut i = 0;
    while i < lemmas.len() {
        if lemmas[i].is_empty() {
            i += 1;
            continue;
        }
        let mut update = false;
        for j in 0..lemmas.len() {
            if i == j {
                continue;
            }
            if lemmas[j].is_empty() {
                continue;
            }
            let (res, diff) = lemmas[i].subsume_execpt_one(&lemmas[j]);
            if res {
                lemmas[j] = Default::default();
                continue;
            } else if let Some(diff) = diff {
                if lemmas[i].len() == lemmas[j].len() {
                    update = true;
                    let mut cube = lemmas[i].cube().clone();
                    cube.retain(|l| *l != diff);
                    assert!(cube.len() + 1 == lemmas[i].len());
                    lemmas[i] = Lemma::new(cube);
                    lemmas[j] = Default::default();
                } else {
                    let mut cube = lemmas[j].cube().clone();
                    cube.retain(|l| *l != !diff);
                    assert!(cube.len() + 1 == lemmas[j].len());
                    lemmas[j] = Lemma::new(cube);
                }
            }
        }
        if !update {
            i += 1;
        }
    }
    lemmas.retain(|l| !l.is_empty());
    lemmas
        .into_iter()
        .map(|l| Clause::from(l.cube().as_slice()))
        .collect()
}
