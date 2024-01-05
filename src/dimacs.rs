use crate::{Clause, Cnf, Lit, Var};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

impl Cnf {
    pub fn from_file<P: AsRef<Path>>(file: P) -> Self {
        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);
        let mut cnf = Cnf::new();
        for line in reader.lines() {
            let line = line.unwrap();
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
