use super::DagCnf;
use crate::Var;
use crate::{Lbool, VarAssign};

impl DagCnf {
    pub fn var_sim(&mut self, n: Var, value: &mut VarAssign) {
        'm: for rel in self.cnf[n].iter() {
            for l in rel.iter() {
                if l.var() != n && value.v(*l) != Lbool::FALSE {
                    continue 'm;
                }
            }
            value.set(rel.last());
            return;
        }
    }
}
