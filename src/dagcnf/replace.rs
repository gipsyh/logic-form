use crate::{DagCnf, Var, VarLMap};

impl DagCnf {
    pub fn replace(&mut self, map: &VarLMap) {
        for (old, new) in map.iter() {
            assert!(*old > new.var());
        }

        for v in Var::CONST..=self.max_var {
            if map.contains_key(v) {
                self.cnf[v].clear();
                self.dep[v].clear();
            }
            for cls in self.cnf[v].iter_mut() {
                for l in cls.iter_mut() {
                    if let Some(new) = map.map_lit(*l) {
                        *l = new;
                    }
                }
            }
            for d in self.dep[v].iter_mut() {
                if let Some(new) = map.map(*d) {
                    *d = new.var();
                }
            }
        }
    }
}
