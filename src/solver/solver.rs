pub trait Problem {
    fn is_solved(&self) -> bool;
}

pub trait Solver<P, S>
where
    P: Problem,
{
    fn solve(&self) -> Option<Solution<P, S>>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<P, S> {
    problem: P,
    steps: Vec<StateTraversal<S>>,
}

impl<P, S> Solution<P, S> {
    pub fn new(problem: P, steps: Vec<StateTraversal<S>>) -> Self {
        Self {
            problem,
            steps,
        }
    }

    pub fn problem(&self) -> &P {
        &self.problem
    }

    pub fn steps(&self) -> usize {
        self.steps.len()
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

    pub fn intermediate_state(previous_state: StateTraversal<S>, state: S) -> Self {
        Self {
            previous_state: Some(Box::new(previous_state)),
            state,
        }
    }

    pub fn final_state(previous_state: StateTraversal<S>, state: S) -> Self {
        Self {
            previous_state: Some(Box::new(previous_state)),
            state,
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn previous_state(&self) -> Option<&StateTraversal<S>> {
        self.previous_state.as_deref()
    }
}
