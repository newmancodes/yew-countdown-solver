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
    instructions: Vec<Instruction<S>>,
}

impl<P, S> Solution<P, S> {
    pub fn new(problem: P, instructions: Vec<Instruction<S>>) -> Self {
        Self {
            problem,
            instructions,
        }
    }

    pub fn problem(&self) -> &P {
        &self.problem
    }

    pub fn number_of_operations(&self) -> usize {
        self.instructions.len() - 1
    }

    pub fn instructions(&self) -> &[Instruction<S>] {
        &self.instructions
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction<S> {
    state: S,
}

impl<S> Instruction<S> {
    pub fn new(state: S) -> Self {
        Self { state }
    }

    pub fn state(&self) -> &S {
        &self.state
    }
}
