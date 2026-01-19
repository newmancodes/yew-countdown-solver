pub trait Solver<'a, T> {
    fn solve(&'a self) -> Option<Solution<'a, T>>;
}

pub struct Solution<'a, T> {
    initial_state: &'a T,
    steps: usize,
}

impl<'a, T> Solution<'a, T> {
    pub fn initial_state(&self) -> &'a T {
        self.initial_state
    }

    pub fn steps(&self) -> usize {
        self.steps
    }
}