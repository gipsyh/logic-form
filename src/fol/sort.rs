#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sort {
    Bv(u32),
    Array(u32, u32),
}

impl Sort {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BvConst {
    c: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayConst {
    c: Vec<BvConst>,
}
