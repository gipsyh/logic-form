use super::DagCnf;
use crate::Lit;
use crate::{Var, VarMap};
use giputils::bitvec::BitVec;
use giputils::hash::GHashSet;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::ops::Index;

impl DagCnf {
    // pub fn sim_var(&mut self, n: Var, value: &mut VarAssign) {
    //     value.set_none(n);
    //     'm: for rel in self.cnf[n].iter() {
    //         for l in rel.iter() {
    //             if l.var() != n && value.v(*l) != Lbool::FALSE {
    //                 continue 'm;
    //             }
    //         }
    //         debug_assert!(rel.last().var() == n);
    //         value.set(rel.last());
    //         return;
    //     }
    // }
}

#[derive(Clone, Debug)]
pub struct DagCnfSimulation {
    sim: VarMap<BitVec>,
}

impl Index<Var> for DagCnfSimulation {
    type Output = BitVec;

    #[inline]
    fn index(&self, var: Var) -> &Self::Output {
        &self.sim[var]
    }
}

impl DagCnfSimulation {
    pub fn new(num_word: usize, dc: &DagCnf) -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let mut sim = VarMap::new_with(dc.max_var());
        sim[Var::CONST] = BitVec::new_with(num_word * BitVec::WORD_SIZE, false);
        let mut leafs = GHashSet::new();
        for v in Var(1)..=dc.max_var() {
            if dc.is_leaf(v) {
                loop {
                    let x = BitVec::new_rand(num_word, &mut rng);
                    if !leafs.contains(&x) {
                        leafs.insert(x.clone());
                        sim[v] = x;
                        break;
                    }
                }
            } else {
                sim[v] = BitVec::new_with(num_word * BitVec::WORD_SIZE, false);
            }
        }
        let mut s = Self { sim };
        s.simulate(dc);
        s
    }

    #[inline]
    pub fn val(&self, lit: Lit) -> BitVec {
        if !lit.polarity() {
            !&self.sim[lit.var()]
        } else {
            self.sim[lit.var()].clone()
        }
    }

    fn simulate_var(&mut self, v: Var, dc: &DagCnf) {
        for rel in dc.cnf[v].iter() {
            let mut sim = self.val(rel[0]);
            let mut vl = rel[0];
            for &l in &rel[1..] {
                if l.var() == v {
                    vl = l;
                }
                if l.polarity() {
                    sim |= &self[l.var()];
                } else {
                    sim |= &!&self[l.var()];
                }
            }
            assert!(vl.var() == v);
            if vl.polarity() {
                self.sim[v] |= &!&sim;
            } else {
                self.sim[v] &= &sim;
            }
        }
    }

    fn simulate(&mut self, dc: &DagCnf) {
        for v in Var(1)..=dc.max_var() {
            if dc.is_leaf(v) {
                continue;
            }
            self.simulate_var(v, dc);
        }
    }
}
