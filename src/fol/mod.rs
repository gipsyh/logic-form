mod map;
pub mod op;
mod sort;
mod term;

pub use sort::*;
pub use term::*;

// pub use op::*;
// pub use term::*;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tests::hash::TERMMAP;

//     #[test]
//     fn test0() {
//         let x = Term::new_var(Sort::BV(2));
//         let y = Term::new_var(Sort::BV(2));
//         let z1 = x.and(&y);
//         let z2 = x.and(&y);
//         assert!(z1 == z2);
//         drop(x);
//         drop(y);
//         drop(z1);
//         drop(z2);
//         assert!(TERMMAP.is_empty());
//     }
// }
