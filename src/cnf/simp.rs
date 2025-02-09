use super::Cnf;
use crate::{Lit, LitMap, LitSet, LitVec, Var, VarSet};
use giputils::{
    grc::Grc,
    hash::GHashSet,
    heap::{BinaryHeap, BinaryHeapCmp},
};
use std::{
    ops::{Index, IndexMut},
    time::Instant,
};

#[derive(Debug, Clone, Default)]
struct Occur {
    occurs: Vec<usize>,
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
    fn lazy_remove(&mut self) {
        self.dirty = true;
        self.size -= 1;
    }

    fn remove(&mut self, cls: usize) {
        assert!(!self.dirty);
        self.size -= 1;
        let len = self.occurs.len();
        self.occurs.retain(|&i| i != cls);
        assert!(len == self.occurs.len() + 1);
    }
}

struct Occurs {
    occurs: LitMap<Occur>,
}

impl Occurs {
    fn new(cnf: &Cnf) -> Self {
        let mut occurs: LitMap<Occur> = LitMap::new_with(cnf.max_var());
        for (i, cls) in cnf.cls.iter().enumerate() {
            for &lit in cls.iter() {
                occurs[lit].add(i);
            }
        }
        Self { occurs }
    }

    #[inline]
    #[allow(unused)]
    fn var_num_occurs(&self, v: Var) -> usize {
        self.occurs[v.lit()].len() + self.occurs[!v.lit()].len()
    }

    #[inline]
    fn bve_cost(&self, v: Var) -> isize {
        let pos = self.occurs[v.lit()].len() as isize;
        let neg = self.occurs[!v.lit()].len() as isize;
        pos * neg - pos - neg
    }
}

impl BinaryHeapCmp<Var> for Occurs {
    fn lge(&self, s: Var, o: Var) -> bool {
        self.bve_cost(s) < self.bve_cost(o)
    }
}

impl Index<Lit> for Occurs {
    type Output = Occur;

    #[inline]
    fn index(&self, index: Lit) -> &Self::Output {
        &self.occurs[index]
    }
}

impl IndexMut<Lit> for Occurs {
    #[inline]
    fn index_mut(&mut self, index: Lit) -> &mut Self::Output {
        &mut self.occurs[index]
    }
}

pub struct CnfSimplify {
    cnf: Cnf,
    qassign: Vec<Lit>,
    qassign_head: usize,
    occurs: Grc<Occurs>,
    frozen: GHashSet<Var>,
    in_bve: bool,
    qbve: BinaryHeap<Var, Occurs>,
    bve_cand: VarSet,
    removed: GHashSet<usize>,
    update: bool,
    subsume_litcand: LitSet,
}

impl CnfSimplify {
    pub fn new(mut cnf: Cnf) -> Self {
        for cls in cnf.iter_mut() {
            cls.sort();
        }
        let occurs = Grc::new(Occurs::new(&cnf));
        let mut qeliminate = BinaryHeap::new(occurs.clone());
        let bve_cand = VarSet::new_with(cnf.max_var());
        let mut subsume_litcand = LitSet::new_with(cnf.max_var());
        for v in Var::CONST..=cnf.max_var() {
            qeliminate.push(v);
        }
        let mut qassign = Vec::new();
        for cls in cnf.iter() {
            if cls.len() == 1 {
                qassign.push(cls[0]);
            }
            for l in cls {
                subsume_litcand.insert(*l);
            }
        }
        Self {
            cnf,
            qassign,
            qassign_head: 0,
            occurs,
            frozen: GHashSet::new(),
            removed: GHashSet::new(),
            qbve: qeliminate,
            bve_cand,
            subsume_litcand,
            in_bve: false,
            update: true,
        }
    }

    pub fn froze(&mut self, v: Var) {
        self.frozen.insert(v);
    }

    fn add_clause(&mut self, cls: &[Lit]) {
        self.update = true;
        assert!(!cls.is_empty());
        if cls.len() == 1 {
            self.qassign.push(cls[0]);
        }
        let i = self.cnf.len();
        self.cnf.add_clause(cls);
        for lit in self.cnf[i].clone() {
            self.occurs[lit].add(i);
            self.qbve.update(lit.var());
            if self.in_bve {
                self.bve_cand.insert(lit.var());
            } else {
                self.qbve.push(lit.var());
            }
            self.subsume_litcand.insert(lit);
        }
    }

    fn remove_clause(&mut self, i: usize) {
        self.update = true;
        for &lit in self.cnf[i].iter() {
            self.occurs[lit].lazy_remove();
            self.qbve.update(lit.var());
            if self.in_bve {
                self.bve_cand.insert(lit.var());
            } else {
                self.qbve.push(lit.var());
            }
        }
        self.removed.insert(i);
    }

    fn remove_lit_in_one(&mut self, lit: Lit, s: usize) {
        self.update = true;
        self.cnf[s].retain(|&l| l != lit);
        assert!(!self.cnf[s].is_empty());
        if self.cnf[s].len() == 1 {
            self.qassign.push(self.cnf[s][0]);
        }
        self.occurs[lit].clean(&self.removed);
        self.occurs[lit].remove(s);
        self.qbve.update(lit.var());
        for l in self.cnf[s].iter() {
            if self.in_bve {
                self.bve_cand.insert(lit.var());
            } else {
                self.qbve.push(lit.var());
            }
            self.subsume_litcand.insert(*l);
        }
    }

    fn remove_lit_in_all(&mut self, lit: Lit) {
        self.update = true;
        for s in self.get_occurs(lit).clone() {
            self.cnf[s].retain(|&l| l != lit);
            assert!(!self.cnf[s].is_empty());
            if self.cnf[s].len() == 1 {
                self.qassign.push(self.cnf[s][0]);
            }
            for l in self.cnf[s].iter() {
                if self.in_bve {
                    self.bve_cand.insert(lit.var());
                } else {
                    self.qbve.push(lit.var());
                }
                self.subsume_litcand.insert(*l);
            }
        }
        self.occurs[lit].clear();
        self.qbve.update(lit.var());
    }

    #[inline]
    fn get_occurs(&mut self, lit: Lit) -> &Vec<usize> {
        self.occurs[lit].clean(&self.removed);
        &self.occurs[lit].occurs
    }

    fn const_simp(&mut self) {
        while self.qassign_head < self.qassign.len() {
            let lit = self.qassign[self.qassign_head];
            for s in self.get_occurs(lit).clone() {
                self.remove_clause(s);
            }
            self.remove_lit_in_all(!lit);
            self.qassign_head += 1;
        }
    }

    fn eliminate(&mut self, v: Var) {
        if self.frozen.contains(&v) {
            return;
        }
        let l = v.lit();
        let origin_cost = self.occurs[l].len() + self.occurs[!l].len();
        let pos = self.get_occurs(l).clone();
        let neg = self.get_occurs(!l).clone();
        let mut new_cnf = Vec::new();
        for x in pos.iter() {
            for y in neg.iter() {
                if let Some(r) = self.cnf[*x].ordered_resolvent(&self.cnf[*y], v) {
                    new_cnf.push(r);
                }
                if new_cnf.len() > origin_cost + 5 {
                    return;
                }
            }
        }
        for n in new_cnf.iter() {
            assert!(!n.is_empty());
        }
        let new_cnf = clause_subsume_simplify(new_cnf);
        if new_cnf.len() > origin_cost {
            return;
        }
        for cls in pos.into_iter().chain(neg) {
            self.remove_clause(cls);
        }
        for mut n in new_cnf {
            n.sort();
            self.add_clause(&n);
        }
    }

    fn bve_simp(&mut self) {
        self.in_bve = true;
        while let Some(v) = self.qbve.pop() {
            self.eliminate(v);
        }
        for &v in self.bve_cand.iter() {
            self.qbve.push(v);
        }
        self.bve_cand.clear();
        self.in_bve = false;
    }

    fn subsume_simp(&mut self) {
        let mut cls_cand = GHashSet::new();
        for l in self.subsume_litcand.elements().to_vec() {
            for s in self.get_occurs(l).clone() {
                cls_cand.insert(s);
            }
        }
        self.subsume_litcand.clear();
        for i in cls_cand {
            if self.removed.contains(&i) {
                continue;
            }
            let best_lit = *self.cnf[i]
                .iter()
                .min_by_key(|l| self.occurs[**l].len())
                .unwrap();
            for j in self.get_occurs(best_lit).clone() {
                if i == j {
                    continue;
                }
                let (res, diff) = self.cnf[i].ordered_subsume_execpt_one(&self.cnf[j]);
                if res {
                    self.remove_clause(j);
                } else if let Some(diff) = diff {
                    if self.cnf[i].len() == self.cnf[j].len() {
                        self.remove_clause(j);
                        self.remove_lit_in_one(diff, i);
                    } else {
                        self.remove_lit_in_one(!diff, j);
                    }
                }
            }
        }
    }

    pub fn simplify(&mut self) -> Cnf {
        let start = Instant::now();
        while self.update {
            self.update = false;
            // dbg!(self.cnf.len() - self.removed.len());
            self.const_simp();
            // dbg!(self.cnf.len() - self.removed.len());
            self.bve_simp();
            // dbg!(self.cnf.len() - self.removed.len());
            self.subsume_simp();
            // dbg!(self.cnf.len() - self.removed.len());
        }
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

fn clause_subsume_simplify(mut cls: Vec<LitVec>) -> Vec<LitVec> {
    cls.sort_by_key(|l| l.len());
    for c in cls.iter_mut() {
        c.sort();
    }
    let mut i = 0;
    while i < cls.len() {
        if cls[i].is_empty() {
            i += 1;
            continue;
        }
        let mut update = false;
        for j in 0..cls.len() {
            if i == j {
                continue;
            }
            if cls[j].is_empty() {
                continue;
            }
            let (res, diff) = cls[i].ordered_subsume_execpt_one(&cls[j]);
            if res {
                cls[j] = Default::default();
                continue;
            } else if let Some(diff) = diff {
                if cls[i].len() == cls[j].len() {
                    update = true;
                    cls[i].retain(|l| *l != diff);
                    cls[j] = Default::default();
                } else {
                    cls[j].retain(|l| *l != !diff);
                }
            }
        }
        if !update {
            i += 1;
        }
    }
    cls.retain(|l| !l.is_empty());
    cls
}
