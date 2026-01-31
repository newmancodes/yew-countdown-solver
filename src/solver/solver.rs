pub trait Solvable {
    fn is_solved(&self) -> bool;
}

pub trait Solver<'a, S>
where
    S: Solvable,
{
    fn solve(&'a self) -> Option<Solution<'a, S>>;
}

pub struct Solution<'a, S> {
    initial_state: &'a S,
    steps: usize,
}

impl<'a, S> Solution<'a, S> {
    pub fn new(initial_state: &'a S, steps: usize) -> Self {
        Self {
            initial_state,
            steps,
        }
    }

    pub fn initial_state(&self) -> &'a S {
        self.initial_state
    }

    pub fn steps(&self) -> usize {
        self.steps
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateTraversal<S> {
    previous_state: Option<Box<StateTraversal<S>>>,
    state: S,
}

impl<S> StateTraversal<S> {
    pub fn initial_state(state: S) -> Self {
        Self {
            previous_state: None,
            state,
        }
    }

    pub fn intermediary(previous_state: StateTraversal<S>, state: S) -> Self {
        Self {
            previous_state: Some(Box::new(previous_state)),
            state,
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }
}
