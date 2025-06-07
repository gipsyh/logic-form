use crate::{Lbool, Lit, Var, VarMap};

#[derive(Default, Clone)]
pub struct VarAssign {
    v: VarMap<Lbool>,
}

impl VarAssign {
    #[inline]
    pub fn new() -> Self {
        VarAssign { v: VarMap::new() }
    }

    #[inline]
    pub fn reserve(&mut self, var: Var) {
        self.v.reserve(var)
    }

    #[inline]
    pub fn v(&self, lit: Lit) -> Lbool {
        Lbool(self.v[lit].0 ^ (!lit.polarity() as u8))
    }

    #[inline]
    pub fn set(&mut self, lit: Lit) {
        self.v[lit] = Lbool(lit.polarity() as u8)
    }

    #[inline]
    pub fn set_none(&mut self, var: Var) {
        self.v[var] = Lbool::NONE
    }
}
