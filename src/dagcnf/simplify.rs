use super::{DagCnf, occur::Occurs};
use crate::{LitMap, LitOrdVec, LitVec, LitVvec, Var, lemmas_subsume_simplify};
use giputils::{allocator::Gallocator, grc::Grc, hash::GHashSet, heap::BinaryHeap};
use std::iter::once;

pub struct DagCnfSimplify {
    cdb: Grc<Gallocator<LitOrdVec>>,
    max_var: Var,
    cnf: LitMap<Vec<u32>>,
    occur: Grc<Occurs>,
    frozen: GHashSet<Var>,
    qbve: BinaryHeap<Var, Occurs>,
}

impl DagCnfSimplify {
    pub fn new(dagcnf: &DagCnf) -> Self {
        let cdb = Grc::new(Gallocator::new());
        let max_var = dagcnf.max_var;
        let occur = Grc::new(Occurs::new_with(max_var, cdb.clone()));
        let cnf = LitMap::new_with(max_var);
        let qbve = BinaryHeap::new(occur.clone());
        let mut res = Self {
            cdb,
            occur,
            max_var,
            cnf,
            frozen: Default::default(),
            qbve,
        };
        for v in Var::CONST..=max_var {
            for mut cls in dagcnf.cnf[v].clone() {
                cls.cls_simp();
                if cls.is_empty() {
                    continue;
                }
                assert!(cls.last().var().eq(&v));
                res.add_rel(cls);
            }
        }
        for v in Var::CONST..=max_var {
            res.qbve.push(v);
        }
        res
    }

    pub fn froze(&mut self, v: Var) {
        self.frozen.insert(v);
    }

    fn add_rel(&mut self, rel: LitVec) {
        let rel = LitOrdVec::new(rel);
        let n = rel.last();
        let relid = self.cdb.alloc(rel);
        self.cnf[n].push(relid);
        for &l in self.cdb[relid].iter() {
            let lv = l.var();
            if lv != n.var() {
                self.occur.add(l, relid);
                self.qbve.down(lv);
            }
        }
    }

    fn remove_rel(&mut self, rels: Vec<u32>) {
        let relset = GHashSet::from_iter(rels.iter().copied());
        let outs = GHashSet::from_iter(rels.iter().map(|&cls| self.cdb[cls].last()));
        for o in outs {
            let mut i = 0;
            while i < self.cnf[o].len() {
                if relset.contains(&self.cnf[o][i]) {
                    let cls = self.cnf[o].swap_remove(i);
                    for &l in self.cdb[cls].iter() {
                        let lv = l.var();
                        if lv != o.var() {
                            self.occur.del(l, cls);
                        }
                    }
                    self.cdb.dealloc(cls);
                } else {
                    i += 1;
                }
            }
        }
    }

    #[inline]
    fn remove_node(&mut self, n: Var) {
        let ln = n.lit();
        assert!(self.occur.num_occur(ln) == 0);
        assert!(self.occur.num_occur(!ln) == 0);
        for &cls in self.cnf[ln].iter().chain(self.cnf[!ln].iter()) {
            for &l in self.cdb[cls].iter() {
                let lv = l.var();
                if lv != n {
                    self.occur.del(l, cls);
                    self.qbve.up(lv);
                }
            }
            self.cdb.dealloc(cls);
        }
        self.cnf[ln].clear();
        self.cnf[!ln].clear();
    }

    fn resolvent(&self, pcnf: &[u32], ncnf: &[u32], pivot: Var, limit: usize) -> Option<LitVvec> {
        let mut res = LitVvec::new();
        for &pcls in pcnf {
            for &ncls in ncnf {
                if let Some(resolvent) = self.cdb[pcls].ordered_resolvent(&self.cdb[ncls], pivot) {
                    res.push(resolvent);
                }
                if res.len() > limit {
                    return None;
                }
            }
        }
        Some(res)
    }

    fn eliminate(&mut self, v: Var) {
        if self.frozen.contains(&v) {
            return;
        }
        let lv = v.lit();
        let ocost = self.occur.num_occur(lv)
            + self.occur.num_occur(!lv)
            + self.cnf[lv].len()
            + self.cnf[!lv].len();
        if ocost == 0 || ocost > 2000 {
            return;
        }
        let (pos, neg) = (self.cnf[lv].clone(), self.cnf[!lv].clone());
        let mut ncost = 0;
        let mut opos = self.occur.get(lv).to_vec();
        let oneg = self.occur.get(!lv).to_vec();
        let Some(respn) = self.resolvent(&pos, &oneg, v, ocost - ncost) else {
            return;
        };
        ncost += respn.len();
        if ncost > ocost {
            return;
        }
        let Some(resnp) = self.resolvent(&neg, &opos, v, ocost - ncost) else {
            return;
        };
        ncost += resnp.len();
        if ncost > ocost {
            return;
        }
        let mut res = respn;
        res.extend(resnp);
        let res = clause_subsume_simplify(res);
        opos.extend(oneg);
        self.remove_rel(opos);
        self.remove_node(v);
        for r in res {
            self.add_rel(r);
        }
    }

    pub fn bve_simplify(&mut self) {
        while let Some(v) = self.qbve.pop() {
            self.eliminate(v);
        }
    }

    fn cls_subsume_check(&mut self, ci: u32) {
        if self.cdb.is_removed(ci) {
            return;
        }
        let best_lit = *self.cdb[ci]
            .iter()
            .min_by_key(|&&l| self.occur.num_occur(l) + self.cnf[l].len())
            .unwrap();
        let mut occur = self.occur.get(best_lit).to_vec();
        occur.extend(self.cnf[best_lit].iter());
        for cj in occur {
            if self.cdb.is_removed(cj) {
                continue;
            }
            if cj == ci {
                continue;
            }
            let (res, diff) = self.cdb[ci].subsume_execpt_one(&self.cdb[cj]);
            if res {
                self.cnf[self.cdb[cj].last()].retain(|&c| c != cj);
                self.cdb.dealloc(cj);
                continue;
            } else if let Some(diff) = diff {
                if self.cdb[ci].len() == self.cdb[cj].len() {
                    assert!(diff.var() != self.cdb[ci].last().var());
                    let mut cube = self.cdb[ci].cube().clone();
                    cube.retain(|l| *l != diff);
                    assert!(cube.last() == self.cdb[ci].last());
                    self.cdb[ci] = LitOrdVec::new(cube);
                    self.cnf[self.cdb[cj].last()].retain(|&c| c != cj);
                    self.cdb.dealloc(cj);
                } else if diff.var() == self.cdb[cj].last().var() {
                    self.cnf[self.cdb[cj].last()].retain(|&c| c != cj);
                    self.cdb.dealloc(cj);
                } else {
                    let mut cube = self.cdb[cj].cube().clone();
                    assert!(cube.last() == self.cdb[cj].last());
                    cube.retain(|l| *l != !diff);
                    self.cdb[cj] = LitOrdVec::new(cube);
                }
            }
        }
    }

    pub fn subsume_simplify(&mut self) {
        for v in Var::CONST..=self.max_var {
            for cls in self.cnf[v.lit()].clone() {
                self.cls_subsume_check(cls);
            }
            for cls in self.cnf[!v.lit()].clone() {
                self.cls_subsume_check(cls);
            }
        }
    }

    pub fn simplify(&mut self) -> DagCnf {
        self.bve_simplify();
        self.subsume_simplify();
        let mut dagcnf = DagCnf::new();
        dagcnf.new_var_to(self.max_var);
        for v in Var(1)..=self.max_var {
            let cnf: Vec<_> = self.cnf[v.lit()]
                .iter()
                .chain(self.cnf[!v.lit()].iter())
                .filter(|&&cls| !self.cdb.is_removed(cls))
                .map(|&cls| self.cdb[cls].cube().clone())
                .collect();
            assert!(cnf.iter().all(|cls| cls.last().var().eq(&v)));
            dagcnf.add_rel(v, &cnf);
        }
        dagcnf
    }
}

fn clause_subsume_simplify(lemmas: LitVvec) -> LitVvec {
    let lemmas: Vec<LitOrdVec> = lemmas.into_iter().map(LitOrdVec::new).collect();
    let lemmas = lemmas_subsume_simplify(lemmas);
    lemmas
        .into_iter()
        .map(|l| LitVec::from(l.cube().as_slice()))
        .collect()
}

impl DagCnf {
    pub fn simplify(&self, frozen: impl Iterator<Item = Var>) -> Self {
        let mut simp = DagCnfSimplify::new(self);
        for v in frozen.chain(once(Var::CONST)) {
            simp.froze(v);
        }
        simp.simplify()
    }
}
