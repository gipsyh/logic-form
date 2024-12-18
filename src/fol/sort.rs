#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sort {
    Bv(u32),
    Array(u32, u32),
}

impl Sort {
    #[inline]
    pub fn bv_len(&self) -> u32 {
        if let Sort::Bv(w) = self {
            *w
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BvConst {
    c: Vec<bool>,
}

impl BvConst {
    #[inline]
    pub fn new(c: &[bool]) -> Self {
        Self { c: c.to_vec() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayConst {
    c: Vec<BvConst>,
}
