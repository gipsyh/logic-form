use crate::{Clause, Lit, Var, VarMap};

#[derive(Debug)]
pub struct DagCnf {
    pub max_var: Var,
    pub cnf: Vec<Clause>,
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
        self.max_var
    }

    #[inline]
    pub fn add_rel(&mut self, v: Var, rel: &[Clause]) {
        self.dep[v] = Vec::from_iter(rel.iter().flatten().map(|l| l.var()).filter(|x| *x != v));
        self.dep[v].sort();
        self.dep[v].dedup();
        self.cnf.extend_from_slice(rel);
    }

    #[inline]
    pub fn add_assign_rel(&mut self, n: Lit, s: Lit) {
        let rel = vec![Clause::from([n, !s]), Clause::from([!n, s])];
        self.add_rel(n.var(), &rel);
    }

    #[inline]
    pub fn add_and_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        let rel = vec![
            Clause::from([x, !n]),
            Clause::from([y, !n]),
            Clause::from([!x, !y, n]),
        ];
        self.add_rel(n.var(), &rel);
    }

    #[inline]
    pub fn add_or_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        let rel = vec![
            Clause::from([!x, n]),
            Clause::from([!y, n]),
            Clause::from([x, y, !n]),
        ];
        self.add_rel(n.var(), &rel);
    }

    #[inline]
    pub fn add_xor_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        let rel = vec![
            Clause::from([!x, y, n]),
            Clause::from([x, !y, n]),
            Clause::from([x, y, !n]),
            Clause::from([!x, !y, !n]),
        ];
        self.add_rel(n.var(), &rel);
    }

    #[inline]
    pub fn add_xnor_rel(&mut self, n: Lit, x: Lit, y: Lit) {
        let rel = vec![
            Clause::from([!x, y, !n]),
            Clause::from([x, !y, !n]),
            Clause::from([x, y, n]),
            Clause::from([!x, !y, n]),
        ];
        self.add_rel(n.var(), &rel);
    }

    #[inline]
    pub fn add_ite_rel(&mut self, n: Lit, c: Lit, t: Lit, e: Lit) {
        let rel = vec![
            Clause::from([t, !c, !n]),
            Clause::from([!t, !c, n]),
            Clause::from([e, c, !n]),
            Clause::from([!e, c, n]),
        ];
        self.add_rel(n.var(), &rel);
    }
}

impl Default for DagCnf {
    fn default() -> Self {
        Self {
            max_var: Var(0),
            cnf: vec![Clause::from([Lit::constant(true)])],
            dep: VarMap::new_with(Var(0)),
        }
    }
}
