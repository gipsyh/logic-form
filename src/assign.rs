use crate::{Lbool, Lit, Var, VarMap};

#[derive(Clone)]
pub struct VarAssign {
    v: VarMap<Lbool>,
}

impl VarAssign {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn new_with(var: Var) -> Self {
        let mut v = VarAssign::new();
        v.reserve(var);
        v
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

impl Default for VarAssign {
    #[inline]
    fn default() -> Self {
        let v = VarMap::new();
        let mut res = Self { v };
        res.reserve(Var::CONST);
        res.set(Lit::constant(true));
        res
    }
}
