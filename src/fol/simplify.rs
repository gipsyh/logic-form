use super::{Term, TermManager, TermResult};
use giputils::hash::GHashMap;

impl Term {
    pub fn simplify(&self, tm: &mut TermManager, map: &mut GHashMap<Term, Term>) -> Term {
        if let Some(res) = map.get(self) {
            return res.clone();
        }
        let simp = if let Some(op_term) = self.try_op_term() {
            let terms: Vec<Term> = op_term.terms.iter().map(|s| s.simplify(tm, map)).collect();
            if let TermResult::Some(new) = op_term.op.simplify(tm, &terms) {
                new
            } else {
                tm.new_op_term(op_term.op.clone(), &terms)
            }
        } else {
            self.clone()
        };
        map.insert(self.clone(), simp);
        map.get(self).unwrap().clone()
    }
}
