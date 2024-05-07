use super::{term::TermType, Sort, Term};
use giputils::grc::Grc;
use std::{collections::HashMap, ops::Deref};

lazy_static::lazy_static! {
    pub static ref TERMMAP: TermMap = TermMap::default();
}

#[derive(Default)]
pub struct TermMap {
    map: HashMap<TermType, usize>,
    sort: HashMap<usize, Sort>,
}

impl TermMap {
    pub fn get(&self, k: &TermType) -> Option<Grc<TermType>> {
        let p = *self.map.get(k)? as *const TermType;
        let v = Grc::from_ptr(p);
        v.increment_count();
        Some(v)
    }

    #[inline]
    pub fn sort(&self, term: &Term) -> Sort {
        *self.sort.get(&(term.inner.as_ptr() as _)).unwrap()
    }

    pub fn insert(&self, k: TermType, v: &Grc<TermType>, sort: Sort) {
        #[allow(invalid_reference_casting)]
        let s = unsafe { &mut *(&self.map as *const _ as *mut HashMap<TermType, usize>) };
        let v = v.as_ptr() as usize;
        assert!(s.insert(k, v).is_none());
        #[allow(invalid_reference_casting)]
        let s = unsafe { &mut *(&self.sort as *const _ as *mut HashMap<usize, Sort>) };
        assert!(s.insert(v, sort).is_none());
    }

    #[inline]
    pub fn remove(&self, term: &TermType) {
        #[allow(invalid_reference_casting)]
        let s = unsafe { &mut *(&self.map as *const _ as *mut HashMap<TermType, usize>) };
        let v = s.remove(term).unwrap();
        #[allow(invalid_reference_casting)]
        let s = unsafe { &mut *(&self.sort as *const _ as *mut HashMap<usize, Sort>) };
        assert!(s.remove(&v).is_some());
    }
}

impl Deref for TermMap {
    type Target = HashMap<TermType, usize>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
