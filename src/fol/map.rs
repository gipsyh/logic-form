// lazy_static::lazy_static! {
//     pub static ref TERMMAP: TermMap = TermMap::default();
// }

// #[derive(Default)]
// pub struct TermMap {
//     map: HashMap<TermInner, Term>,
// }

// impl TermMap {
//     #[inline]
//     pub fn get(&self, k: &TermInner) -> Option<&Term> {
//         self.map.get(k).clone()
//     }

//     #[inline]
//     pub fn insert(&mut self, k: TermInner) -> Term {
//         if let Some(t) = self.map.get(&k) {
//             return t.clone();
//         }
//         let t = Term {
//             inner: Grc::new(k.clone()),
//         };
//         self.map.insert(k, t.clone());
//         t
//     }

//     //     #[inline]
//     //     pub fn remove(&self, _term: &TermInner) {
//     //         todo!();
//     //         // #[allow(invalid_reference_casting)]
//     //         // let s = unsafe { &mut *(&self.map as *const _ as *mut HashMap<TermInner, usize>) };
//     //         // let v = s.remove(term).unwrap();
//     //     }
// }
