use super::{Sort, Term};

#[derive(Debug, Copy, Clone, strum::EnumString, strum::Display, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum UniOpType {
    Not,
    Inc,
    Dec,
    Neg,
    Redand,
    Redor,
    Redxor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniOp {
    pub ty: UniOpType,
    pub a: Term,
}

impl UniOp {
    pub fn sort(&self) -> Sort {
        let a = self.a.sort();
        match a {
            Sort::Bool => match self.ty {
                UniOpType::Not => a,
                UniOpType::Inc => todo!(),
                UniOpType::Dec => todo!(),
                UniOpType::Neg => todo!(),
                UniOpType::Redand => todo!(),
                UniOpType::Redor => todo!(),
                UniOpType::Redxor => todo!(),
            },
            Sort::BV(_) => match self.ty {
                UniOpType::Not | UniOpType::Inc | UniOpType::Dec | UniOpType::Neg => a,
                UniOpType::Redand | UniOpType::Redor | UniOpType::Redxor => Sort::Bool,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, strum::EnumString, strum::Display, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum BiOpType {
    Iff,
    Implies,
    Eq,
    Neq,
    Sgt,
    Ugt,
    Sgte,
    Ugte,
    Slt,
    Ult,
    Slte,
    Ulte,
    And,
    Nand,
    Nor,
    Or,
    Xnor,
    Xor,
    Rol,
    Ror,
    Sll,
    Sra,
    Srl,
    Add,
    Mul,
    Sdiv,
    Udiv,
    Smod,
    Srem,
    Urem,
    Sub,
    Saddo,
    Uaddo,
    Sdivo,
    Udivo,
    Smulo,
    Umulo,
    Ssubo,
    Usubo,
    Concat,
    Read,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BiOp {
    pub ty: BiOpType,
    pub a: Term,
    pub b: Term,
}

impl BiOp {
    pub fn sort(&self) -> Sort {
        let a = self.a.sort();
        let b = self.b.sort();
        match a {
            Sort::Bool => match self.ty {
                BiOpType::Iff
                | BiOpType::Implies
                | BiOpType::Eq
                | BiOpType::Neq
                | BiOpType::And
                | BiOpType::Nand
                | BiOpType::Nor
                | BiOpType::Or
                | BiOpType::Xnor
                | BiOpType::Xor => {
                    assert!(a == b);
                    a
                }
                BiOpType::Sgt => todo!(),
                BiOpType::Ugt => todo!(),
                BiOpType::Sgte => todo!(),
                BiOpType::Ugte => todo!(),
                BiOpType::Slt => todo!(),
                BiOpType::Ult => todo!(),
                BiOpType::Slte => todo!(),
                BiOpType::Ulte => todo!(),
                BiOpType::Rol => todo!(),
                BiOpType::Ror => todo!(),
                BiOpType::Sll => todo!(),
                BiOpType::Sra => todo!(),
                BiOpType::Srl => todo!(),
                BiOpType::Add => todo!(),
                BiOpType::Mul => todo!(),
                BiOpType::Sdiv => todo!(),
                BiOpType::Udiv => todo!(),
                BiOpType::Smod => todo!(),
                BiOpType::Srem => todo!(),
                BiOpType::Urem => todo!(),
                BiOpType::Sub => todo!(),
                BiOpType::Saddo => todo!(),
                BiOpType::Uaddo => todo!(),
                BiOpType::Sdivo => todo!(),
                BiOpType::Udivo => todo!(),
                BiOpType::Smulo => todo!(),
                BiOpType::Umulo => todo!(),
                BiOpType::Ssubo => todo!(),
                BiOpType::Usubo => todo!(),
                BiOpType::Concat => todo!(),
                BiOpType::Read => todo!(),
            },
            Sort::BV(aw) => match self.ty {
                BiOpType::Iff
                | BiOpType::Implies
                | BiOpType::Eq
                | BiOpType::Neq
                | BiOpType::Sgt
                | BiOpType::Ugt
                | BiOpType::Sgte
                | BiOpType::Ugte
                | BiOpType::Slt
                | BiOpType::Ult
                | BiOpType::Slte
                | BiOpType::Ulte => {
                    assert!(a == b);
                    Sort::Bool
                }
                BiOpType::And
                | BiOpType::Nand
                | BiOpType::Nor
                | BiOpType::Or
                | BiOpType::Xnor
                | BiOpType::Xor => {
                    assert!(a == b);
                    a
                }
                BiOpType::Rol => todo!(),
                BiOpType::Ror => todo!(),
                BiOpType::Sll => todo!(),
                BiOpType::Sra => todo!(),
                BiOpType::Srl => todo!(),
                BiOpType::Add => todo!(),
                BiOpType::Mul => todo!(),
                BiOpType::Sdiv => todo!(),
                BiOpType::Udiv => todo!(),
                BiOpType::Smod => todo!(),
                BiOpType::Srem => todo!(),
                BiOpType::Urem => todo!(),
                BiOpType::Sub => todo!(),
                BiOpType::Saddo => todo!(),
                BiOpType::Uaddo => todo!(),
                BiOpType::Sdivo => todo!(),
                BiOpType::Udivo => todo!(),
                BiOpType::Smulo => todo!(),
                BiOpType::Umulo => todo!(),
                BiOpType::Ssubo => todo!(),
                BiOpType::Usubo => todo!(),
                BiOpType::Concat => todo!(),
                BiOpType::Read => todo!(),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, strum::EnumString, strum::Display, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum TriOpType {
    Ite,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TriOp {
    pub ty: TriOpType,
    pub a: Term,
    pub b: Term,
    pub c: Term,
}

impl TriOp {
    pub fn sort(&self) -> Sort {
        let a = self.a.sort();
        let b = self.b.sort();
        let c = self.c.sort();
        match a {
            Sort::Bool => match self.ty {
                TriOpType::Ite => {
                    assert!(b == c);
                    b
                }
                TriOpType::Write => todo!(),
            },
            Sort::BV(_) => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone, strum::EnumString, strum::Display, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum ExtOpType {
    Sext,
    Uext,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtOp {
    pub ty: ExtOpType,
    pub a: Term,
    pub length: u32,
}

impl ExtOp {
    pub fn sort(&self) -> Sort {
        let a = self.a.sort();
        Sort::BV(
            self.length
                + match a {
                    Sort::Bool => 1,
                    Sort::BV(w) => w,
                },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SliceOp {
    pub a: Term,
    pub upper: u32,
    pub lower: u32,
}

impl SliceOp {
    pub fn sort(&self) -> Sort {
        Sort::BV(self.upper - self.lower + 1)
    }
}
