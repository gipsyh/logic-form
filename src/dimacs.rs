use crate::{Clause, Cnf, Lit, Var};
use std::{fs::read_to_string, path::Path};

impl Cnf {
    pub fn from_dimacs_file<P: AsRef<Path>>(file: P) -> Self {
        let str = read_to_string(file).unwrap();
        Self::from_dimacs_str(&str)
    }

    pub fn from_dimacs_str(str: &str) -> Self {
        let mut cnf = Cnf::new();
        for line in str.lines() {
            let symbols: Vec<&str> = line.split_whitespace().collect();
            match symbols[0] {
                "p" => (),
                "c" => (),
                _ => {
                    let mut clause: Vec<i32> =
                        symbols.iter().map(|&s| s.parse::<i32>().unwrap()).collect();
                    assert!(clause.pop().unwrap() == 0);
                    cnf.push(Clause::from_iter(
                        clause
                            .into_iter()
                            .map(|lit| Lit::new(Var::new(lit.unsigned_abs() as _), lit > 0)),
                    ));
                }
            }
        }
        cnf
    }
}
