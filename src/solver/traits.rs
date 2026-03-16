pub trait Problem {
    fn is_solved(&self) -> bool;
}

pub trait Solver<P, S>
where
    P: Problem,
{
    fn solve(&self) -> Option<Solution<P, S>>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Operation {
    pub left: u32,
    pub operator: Operator,
    pub right: u32,
    pub result: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

    pub fn number_of_operations(&self) -> usize {
        self.instructions.len() - 1
    }

    pub fn instructions(&self) -> &[Instruction<S>] {
        &self.instructions
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction<S> {
    state: S,
    operation: Option<Operation>,
}

impl<S> Instruction<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            operation: None,
        }
    }

    pub fn with_operation(state: S, operation: Operation) -> Self {
        Self {
            state,
            operation: Some(operation),
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn operation(&self) -> Option<&Operation> {
        self.operation.as_ref()
    }
}
