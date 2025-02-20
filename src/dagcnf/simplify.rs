use super::DagCnf;
use crate::{LitVec, LitVvec, Var, VarMap};
use giputils::{
    grc::Grc,
    hash::GHashSet,
    heap::{BinaryHeap, BinaryHeapCmp},
};
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct Out(VarMap<GHashSet<Var>>);

impl Deref for Out {
    type Target = VarMap<GHashSet<Var>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Out {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<Var> for Out {
    type Output = GHashSet<Var>;

    #[inline]
    fn index(&self, index: Var) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<Var> for Out {
    #[inline]
    fn index_mut(&mut self, index: Var) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl BinaryHeapCmp<Var> for Out {
    #[inline]
    fn lge(&self, s: Var, o: Var) -> bool {
        self[s].len() < self[o].len()
    }
}

pub struct DagCnfSimplify {
    max_var: Var,
    cnf: VarMap<Vec<Grc<LitVec>>>,
    dep: VarMap<GHashSet<Var>>,
    out: Grc<Out>,
    frozen: GHashSet<Var>,
    qbve: BinaryHeap<Var, Out>,
}

impl DagCnfSimplify {
    pub fn new(dagcnf: &DagCnf) -> Self {
        let max_var = dagcnf.max_var;
        let mut cnf: VarMap<Vec<_>> = VarMap::new_with(max_var);
        for v in Var::CONST..=max_var {
            for cls in dagcnf.cnf[v].iter() {
                let mut cls = cls.clone();
                cls.sort();
                cnf[v].push(Grc::new(cls));
            }
        }
        let mut dep: VarMap<GHashSet<_>> = VarMap::new_with(max_var);
        let mut out: VarMap<GHashSet<_>> = VarMap::new_with(max_var);
        for v in Var::CONST..=max_var {
            for d in dagcnf.dep[v].iter() {
                out[*d].insert(v);
                dep[v].insert(*d);
            }
        }
        let out = Grc::new(Out(out));
        let mut qbve = BinaryHeap::new(out.clone());
        for v in Var::CONST..=max_var {
            qbve.push(v);
        }
        Self {
            max_var: dagcnf.max_var,
            cnf,
            dep,
            out,
            frozen: GHashSet::new(),
            qbve,
        }
    }

    pub fn froze(&mut self, v: Var) {
        self.frozen.insert(v);
    }

    fn add_rel(&mut self, n: Var, rel: &[LitVec]) {
        for cls in rel.iter() {
            for l in cls {
                let lv = l.var();
                if lv != n {
                    self.dep[n].insert(lv);
                    self.out[lv].insert(n);
                    self.qbve.update(lv);
                }
            }
            let mut cls = Grc::new(cls.clone());
            cls.sort();
            self.cnf[n].push(cls);
        }
    }

    fn bve_cost(&self, v: Var) -> usize {
        let res: usize = self.out[v]
            .iter()
            .map(|o| {
                self.cnf[*o]
                    .iter()
                    .filter(|cls| cls.iter().any(|l| l.var() == v))
                    .count()
            })
            .sum();
        res + self.cnf[v].len()
    }

    fn pol_filter(&self, n: Var, v: Var) -> (Vec<Grc<LitVec>>, Vec<Grc<LitVec>>) {
        let vl = v.lit();
        let pos: Vec<_> = self.cnf[n]
            .iter()
            .filter(|cls| cls.iter().any(|l| *l == vl))
            .cloned()
            .collect();
        let neg: Vec<_> = self.cnf[n]
            .iter()
            .filter(|cls| cls.iter().any(|l| *l == !vl))
            .cloned()
            .collect();
        (pos, neg)
    }

    fn eliminate(&mut self, v: Var) {
        if self.frozen.contains(&v) {
            return;
        }
        if self.dep[v].is_empty() {
            return;
        }
        let ocost = self.bve_cost(v);
        let (pos, neg) = self.pol_filter(v, v);
        let mut ncost = 0;
        let mut res = Vec::new();
        let out = self.out[v].clone();
        for &o in out.iter() {
            let (opos, oneg) = self.pol_filter(o, v);
            let mut res0 = resolvent(&pos, &oneg, v);
            ncost += res0.len();
            if ncost > ocost {
                return;
            }
            let res1 = resolvent(&opos, &neg, v);
            ncost += res1.len();
            if ncost > ocost {
                return;
            }
            res0.extend(res1);
            let o = res0.len();
            res0.subsume_simplify();
            if res0.len() < o {
                ncost -= o - res0.len();
            }
            res.push(res0);
        }
        for (&o, rel) in out.iter().zip(res) {
            self.cnf[o].retain(|cls| !cls.iter().any(|l| l.var() == v));
            self.dep[o].remove(&v);
            self.add_rel(o, &rel);
        }
        self.dep[v].clear();
        self.out[v].clear();
        self.cnf[v].clear();
        self.qbve.update(v);
    }

    pub fn bve_simplify(&mut self) {
        while let Some(v) = self.qbve.pop() {
            self.eliminate(v);
        }
    }

    pub fn simplify(&mut self) -> DagCnf {
        self.bve_simplify();
        let mut dagcnf = DagCnf::new();
        dagcnf.new_var_to(self.max_var);
        for v in Var(1)..=self.max_var {
            let cnf: Vec<_> = self.cnf[v].iter().map(|cls| cls.deref().clone()).collect();
            dagcnf.add_rel(v, &cnf);
        }
        dagcnf
    }
}

fn resolvent(pcnf: &[Grc<LitVec>], ncnf: &[Grc<LitVec>], pivot: Var) -> LitVvec {
    let mut res = LitVvec::new();
    for pcls in pcnf.iter() {
        for ncls in ncnf.iter() {
            if let Some(resolvent) = pcls.ordered_resolvent(ncls, pivot) {
                res.push(resolvent);
            }
        }
    }
    res
}
