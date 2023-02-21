// trait ManagedVecTrait<T> {
//     type I;
//     fn add(&mut self, x: T) -> Self::I;
//     fn get(&self, id: &Self::I) -> &T;
//     fn set(&mut self, id: &Self::I, x: T);
// }

pub struct ManagedVec<T> {
    vars: Vec<T>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Idx(usize);

impl<T> ManagedVec<T> {
    pub fn new() -> ManagedVec<T> {
        ManagedVec { vars: Vec::new() }
    }

    pub fn add<F: Fn(Idx) -> T>(&mut self, f: F) -> Idx {
        let next_idx = Idx(self.vars.len());
        self.vars.push(f(next_idx));
        next_idx
    }

    pub fn get(&self, idx: Idx) -> &T {
        let Idx(idx) = idx;
        self.vars
            .get(idx)
            .unwrap_or_else(|| panic!("No such idx: {}", idx))
    }

    pub fn set(&mut self, idx: Idx, x: T) {
        let Idx(idx) = idx;
        match self.vars.get_mut(idx) {
            Some(cur) => *cur = x,
            None => panic!("No such idx: {}", idx),
        }
    }
}
