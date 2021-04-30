use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Bisect<T: PartialOrd + Clone>(Vec<T>);

impl<T: PartialOrd + Clone> Bisect<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn bisect_left(&self, item: &T) -> usize {
        let index = self
            .0
            .binary_search_by(|v| v.partial_cmp(&item).expect("Couldn't compare values"));

        match index {
            Ok(x) => x,
            Err(x) => x,
        }
    }

    pub fn bisect_right(&self, item: &T) -> usize {
        let index = self
            .0
            .binary_search_by(|v| v.partial_cmp(&item).expect("Couldn't compare values"));

        match index {
            Ok(x) => x + 1,
            Err(x) => x,
        }
    }

    pub fn append(&mut self, item: T) {
        let index = self.bisect_left(&item);
        self.0.insert(index, item);
    }
}

impl<T: PartialOrd + Clone> Deref for Bisect<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T: PartialOrd + Clone> DerefMut for Bisect<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}
