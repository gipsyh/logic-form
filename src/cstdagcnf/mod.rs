mod bva;
pub use bva::*;

use crate::{Cnf, DagCnf};

#[derive(Debug, Clone)]
pub struct CstDagCnf {
    pub dag: DagCnf,
    pub cst: Cnf,
}
