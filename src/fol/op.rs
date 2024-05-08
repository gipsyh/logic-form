use super::Term;

#[derive(Debug, Copy, Clone, strum::EnumString, strum::Display, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum UniOpType {
    Not,
    Inc,
    Dec,
    Neg,
    // Redand,
    // Redor,
    // Redxor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniOp {
    pub ty: UniOpType,
    pub a: Term,
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

// /// Slice operation node.
// #[derive(Debug, Clone)]
// pub struct SliceOp {
//     /// Result sort.
//     pub sid: Sid,
//     /// Operand right-side node id.
//     pub a: Rnid,
//     /// Upper bit of slice (inclusive).
//     ///
//     /// Guaranteed to be greater or equal to lower bit after parsing.
//     pub upper_bit: u32,
//     /// Lower bit of slice (inclusive).
//     ///
//     /// Guaranteed to be lesser or equal to upper bit after parsing.
//     pub lower_bit: u32,
// }
