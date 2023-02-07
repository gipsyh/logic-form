use std::{
    fmt::{Debug, Display},
    ops::{Add, Deref, DerefMut, Not},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Var(u32);

impl From<u32> for Var {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<i32> for Var {
    fn from(value: i32) -> Self {
        Self(value as u32)
    }
}

impl From<usize> for Var {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<Var> for u32 {
    fn from(value: Var) -> Self {
        value.0
    }
}

impl From<Var> for usize {
    fn from(value: Var) -> Self {
        value.0 as usize
    }
}

impl Deref for Var {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lit(u32);

impl From<Var> for Lit {
    fn from(value: Var) -> Self {
        Self(value.0 + value.0)
    }
}

impl From<Lit> for u32 {
    fn from(val: Lit) -> Self {
        val.0
    }
}

impl From<Lit> for usize {
    fn from(val: Lit) -> Self {
        val.0 as usize
    }
}

impl Lit {
    pub fn new(var: Var, compl: bool) -> Self {
        Lit(var.0 + var.0 + compl as u32)
    }

    pub fn var(&self) -> Var {
        Var(self.0 >> 1)
    }

    pub fn compl(&self) -> bool {
        self.0 & 1 > 0
    }
}

impl Not for Lit {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.0 ^= 1;
        self
    }
}

impl Debug for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.compl() {
            write!(f, "-{}", self.var())
        } else {
            write!(f, "{}", self.var())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Clause {
    lits: Vec<Lit>,
}

impl Clause {
    pub fn new() -> Self {
        Clause { lits: Vec::new() }
    }
}

impl Default for Clause {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Clause {
    type Target = Vec<Lit>;

    fn deref(&self) -> &Self::Target {
        &self.lits
    }
}

impl DerefMut for Clause {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lits
    }
}

impl<const N: usize> From<[Lit; N]> for Clause {
    fn from(s: [Lit; N]) -> Self {
        Self {
            lits: <[Lit]>::into_vec(Box::new(s)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    lits: Vec<Lit>,
}

impl Cube {
    pub fn new() -> Self {
        Cube { lits: Vec::new() }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Cube {
    type Target = Vec<Lit>;

    fn deref(&self) -> &Self::Target {
        &self.lits
    }
}

impl DerefMut for Cube {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lits
    }
}

impl Not for Cube {
    type Output = Clause;

    fn not(self) -> Self::Output {
        let lits = self.lits.iter().map(|lit| !*lit).collect();
        Clause { lits }
    }
}

#[derive(Clone, Debug)]
pub struct Cnf {
    clauses: Vec<Clause>,
}

impl Cnf {
    pub fn new() -> Self {
        Self {
            clauses: Vec::new(),
        }
    }

    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
}

impl Default for Cnf {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Cnf {
    type Target = Vec<Clause>;

    fn deref(&self) -> &Self::Target {
        &self.clauses
    }
}

impl DerefMut for Cnf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clauses
    }
}

#[derive(Clone, Debug)]
pub struct Dnf {
    cubes: Vec<Cube>,
}

impl Dnf {
    pub fn new() -> Self {
        Self { cubes: Vec::new() }
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.push(cube);
    }
}

impl Default for Dnf {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Dnf {
    type Target = Vec<Cube>;

    fn deref(&self) -> &Self::Target {
        &self.cubes
    }
}

impl DerefMut for Dnf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cubes
    }
}

impl Add for Dnf {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.cubes.append(&mut rhs.cubes);
        self
    }
}

impl Not for Dnf {
    type Output = Cnf;

    fn not(self) -> Self::Output {
        let mut cnf = Cnf::new();
        for cube in self.cubes {
            cnf.add_clause(!cube);
        }
        cnf
    }
}
