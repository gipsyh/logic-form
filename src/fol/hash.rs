// use super::{term::TermInner, Sort, Term};
// use giputils::grc::Grc;
// use std::{collections::HashMap, ops::Deref};

// lazy_static::lazy_static! {
//     pub static ref TERMMAP: TermMap = TermMap::default();
// }

use ahash::HashMap;
use super::term::Term;

#[derive(Default)]
pub struct TermMap {
    map: HashMap<Term, usize>,
}

// impl TermMap {
//     #[inline]
//     pub fn get(&self, k: &TermInner) -> Option<Grc<TermInner>> {
//         let p = *self.map.get(k)? as *const TermInner;
//         let v = Grc::from_ptr(p);
//         v.increment_count();
//         Some(v)
//     }

//     #[inline]
//     pub fn insert(&self, k: TermInner, v: &Grc<TermInner>) {
//         #[allow(invalid_reference_casting)]
//         let s = unsafe { &mut *(&self.map as *const _ as *mut HashMap<TermInner, usize>) };
//         let v = v.as_ptr() as usize;
//         assert!(s.insert(k, v).is_none());
//         #[allow(invalid_reference_casting)]
//         let s = unsafe { &mut *(&self.sort as *const _ as *mut HashMap<usize, Sort>) };
//         assert!(s.insert(v, sort).is_none());
//     }

//     #[inline]
//     pub fn remove(&self, _term: &TermInner) {
//         todo!();
//         // #[allow(invalid_reference_casting)]
//         // let s = unsafe { &mut *(&self.map as *const _ as *mut HashMap<TermInner, usize>) };
//         // let v = s.remove(term).unwrap();
//     }
// }

// impl Deref for TermMap {
//     type Target = HashMap<TermInner, usize>;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.map
//     }
// }
