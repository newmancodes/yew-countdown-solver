#[cfg(test)]
use crate::game::board::BoardBuilder;
use crate::game::board::{Board, BoardAdjuster};
use crate::game::model::Game;
use crate::solver::traits::{Instruction, Operation, Operator, Problem, Solution, Solver};
use std::collections::HashSet;

#[derive(Debug)]
pub struct IterativeDeepeningSolver<'a, T> {
    initial_state: &'a T,
    maximum_depth: usize,
}

impl<'a> IterativeDeepeningSolver<'a, Game> {
    pub fn new(game: &'a Game) -> Self {
        Self {
            initial_state: game,
            maximum_depth: 6,
        }
    }

    fn generate_children(board: &Board) -> Vec<(Board, Operation)> {
        let mut children = Vec::<(Board, Operation)>::new();

        for i in 0..(board.numbers().len() - 1) {
            for j in i + 1..board.numbers().len() {
                let left = board.numbers()[i];
                let right = board.numbers()[j];

                // Addition
                let result = left + right;
                children.push((
                    BoardAdjuster::from(board)
                        .remove_number(left)
                        .remove_number(right)
                        .add_number(result)
                        .build(),
                    Operation {
                        left,
                        operator: Operator::Add,
                        right,
                        result,
                    },
                ));

                // Multiply
                let result = left * right;
                children.push((
                    BoardAdjuster::from(board)
                        .remove_number(left)
                        .remove_number(right)
                        .add_number(result)
                        .build(),
                    Operation {
                        left,
                        operator: Operator::Multiply,
                        right,
                        result,
                    },
                ));

                // Subtraction (larger - smaller, skip if equal)
                if left != right {
                    let (bigger, smaller) = if left > right {
                        (left, right)
                    } else {
                        (right, left)
                    };
                    let result = bigger - smaller;
                    children.push((
                        BoardAdjuster::from(board)
                            .remove_number(bigger)
                            .remove_number(smaller)
                            .add_number(result)
                            .build(),
                        Operation {
                            left: bigger,
                            operator: Operator::Subtract,
                            right: smaller,
                            result,
                        },
                    ));
                }

                // Division (larger / smaller, only when result is integer).
                // Guard: skip if either operand is zero. Currently unreachable
                // because initial board numbers are all >= 1 and subtraction of
                // equal operands is skipped above, but this protects against
                // future changes that could introduce zero onto the board.
                if left == 0 || right == 0 {
                    // no valid division possible; skip
                } else if left == right {
                    children.push((
                        BoardAdjuster::from(board)
                            .remove_number(left)
                            .remove_number(right)
                            .add_number(1)
                            .build(),
                        Operation {
                            left,
                            operator: Operator::Divide,
                            right,
                            result: 1,
                        },
                    ));
                } else if left > right && left.is_multiple_of(right) {
                    let result = left / right;
                    children.push((
                        BoardAdjuster::from(board)
                            .remove_number(left)
                            .remove_number(right)
                            .add_number(result)
                            .build(),
                        Operation {
                            left,
                            operator: Operator::Divide,
                            right,
                            result,
                        },
                    ));
                } else if right.is_multiple_of(left) {
                    let result = right / left;
                    children.push((
                        BoardAdjuster::from(board)
                            .remove_number(left)
                            .remove_number(right)
                            .add_number(result)
                            .build(),
                        Operation {
                            left: right,
                            operator: Operator::Divide,
                            right: left,
                            result,
                        },
                    ));
                }
            }
        }

        children
    }

    fn calculate_child_depth(&self, board: &Board) -> usize {
        self.initial_state.board().numbers().len() - board.numbers().len()
    }
}

impl<'a> Solver<Game, Board> for IterativeDeepeningSolver<'a, Game> {
    fn solve(&self) -> Option<Solution<Game, Board>> {
        if self.initial_state.is_solved() {
            // Simple solution just shows the start and end states
            let initial_state = Instruction::new(self.initial_state.board().clone());
            let instructions = vec![initial_state];
            return Some(Solution::new(self.initial_state.clone(), instructions));
        }

        let mut depth_limit = 1;
        let initial_board = self.initial_state.board();

        while depth_limit <= self.maximum_depth {
            tracing::info!("Depth limit: {}", depth_limit);
            let mut frontier = Vec::<StateTraversal<Board>>::default();
            let mut explored = HashSet::<Board>::default();

            frontier.push(StateTraversal::initial_state(initial_board.clone()));

            while let Some(candidate) = frontier.pop() {
                explored.insert(candidate.state().clone());

                if candidate
                    .state()
                    .numbers()
                    .contains(&(self.initial_state.target() as u32))
                {
                    // A solution has the start, intermediate and end states in order
                    let mut instructions = Vec::with_capacity(depth_limit + 2);
                    if let Some(op) = candidate.operation.clone() {
                        instructions
                            .push(Instruction::with_operation(candidate.state().clone(), op));
                    } else {
                        instructions.push(Instruction::new(candidate.state().clone()));
                    }
                    let mut previous_state = candidate.previous_state();
                    while let Some(visited_state) = previous_state {
                        if let Some(op) = visited_state.operation.clone() {
                            instructions.push(Instruction::with_operation(
                                visited_state.state().clone(),
                                op,
                            ));
                        } else {
                            instructions.push(Instruction::new(visited_state.state().clone()));
                        }
                        previous_state = visited_state.previous_state();
                    }
                    instructions.reverse();
                    return Some(Solution::new(self.initial_state.clone(), instructions));
                }

                for (child_candidate, operation) in Self::generate_children(candidate.state()) {
                    if self.calculate_child_depth(&child_candidate) < depth_limit
                        && !explored.contains(&child_candidate)
                        && !frontier
                            .iter()
                            .any(|traversal| traversal.state() == &child_candidate)
                    {
                        frontier.push(StateTraversal::intermediate_state(
                            candidate.clone(),
                            child_candidate,
                            operation,
                        ));
                    }
                }
            }

            depth_limit += 1;
        }

        None
    }
}

#[derive(Debug, PartialEq, Clone)]
struct StateTraversal<S> {
    previous_state: Option<Box<StateTraversal<S>>>,
    state: S,
    operation: Option<Operation>,
}

impl<S> StateTraversal<S> {
    pub fn initial_state(state: S) -> Self {
        Self {
            previous_state: None,
            state,
            operation: None,
        }
    }

    pub fn intermediate_state(
        previous_state: StateTraversal<S>,
        state: S,
        operation: Operation,
    ) -> Self {
        Self {
            previous_state: Some(Box::new(previous_state)),
            state,
            operation: Some(operation),
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn previous_state(&self) -> Option<&StateTraversal<S>> {
        self.previous_state.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! game {
        ($target:expr, $($num:expr),+ $(,)?) => {{
            let mut builder = BoardBuilder::new();
            $(
                builder = builder.add_number($num).unwrap();
            )+
            let board = builder.build().unwrap();
            Game::new(board, $target).unwrap()
        }};
    }

    #[test]
    fn impossible_game_is_not_solvable() {
        let game = game!(824, 3, 7, 6, 2, 1, 7);

        let solver = IterativeDeepeningSolver::new(&game);

        assert!(solver.solve().is_none());
    }

    #[test]
    fn already_solved_game_is_solved() {
        let game = game!(100, 1, 2, 3, 4, 5, 100);

        let solver = IterativeDeepeningSolver::new(&game);

        let solution = solver.solve();

        assert!(
            solution.is_some(),
            "Expected solution for already solved game"
        );

        let solution = solution.unwrap();

        assert_eq!(solution.number_of_operations(), 0);
    }

    #[test]
    fn solvable_games_are_solved_in_the_expected_number_of_operations() {
        let games_with_expected_solution_steps = [
            (game!(12, 1, 2, 3, 4, 5, 6), 1),
            (game!(350, 1, 4, 4, 5, 6, 50), 2),
            (game!(410, 1, 3, 3, 8, 9, 50), 3),
            (game!(277, 2, 3, 3, 5, 6, 75), 4),
            (game!(831, 1, 10, 25, 50, 75, 100), 5),
        ];

        for (game, expected_solution_steps) in games_with_expected_solution_steps {
            let solver = IterativeDeepeningSolver::new(&game);

            let solution = solver.solve();

            assert!(solution.is_some(), "Expected solution for game {:?}", game);

            let solution = solution.unwrap();

            assert_eq!(
                solution.number_of_operations(),
                expected_solution_steps,
                "Wrong solution steps for {:?} expected {} received {}",
                game,
                expected_solution_steps,
                solution.number_of_operations()
            );
        }
    }

    #[test]
    fn solution_instructions_have_operations_for_all_but_initial() {
        let game = game!(12, 1, 2, 3, 4, 5, 6);

        let solver = IterativeDeepeningSolver::new(&game);
        let solution = solver.solve().expect("Expected a solution");

        let instructions = solution.instructions();
        assert!(
            instructions[0].operation().is_none(),
            "Initial instruction should have no operation"
        );
        for instruction in &instructions[1..] {
            assert!(
                instruction.operation().is_some(),
                "Non-initial instructions should have an operation"
            );
        }
    }

    #[test]
    fn generate_children_never_produces_a_board_containing_zero() {
        // Exhaustively expand children up to depth 5 from several representative
        // boards and assert that zero never appears on any resulting board.
        let starting_boards = [
            game!(999, 1, 1, 2, 2, 3, 3),      // all small, with duplicates
            game!(999, 1, 2, 3, 4, 5, 6),      // all small, no duplicates
            game!(999, 25, 50, 75, 100, 1, 2), // heavy large numbers
            game!(999, 5, 5, 10, 10, 8, 8),    // paired duplicates
        ];

        for game in &starting_boards {
            let mut frontier = vec![game.board().clone()];
            let mut visited = HashSet::<Board>::new();

            while let Some(board) = frontier.pop() {
                if visited.contains(&board) {
                    continue;
                }
                visited.insert(board.clone());

                // The core assertion: no board should ever contain zero
                assert!(
                    !board.numbers().contains(&0),
                    "Board {:?} contains zero, generated from game {:?}",
                    board,
                    game
                );

                // Only expand if the board has at least 2 numbers (needed for pairing)
                if board.numbers().len() >= 2 {
                    for (child_board, _operation) in
                        IterativeDeepeningSolver::generate_children(&board)
                    {
                        if !visited.contains(&child_board) {
                            frontier.push(child_board);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn generate_children_skips_division_when_board_contains_zero() {
        // Synthetically construct a board with zero via BoardAdjuster to verify
        // the defensive guard prevents any division operations involving zero.
        let base_board = BoardBuilder::new()
            .add_number(6)
            .unwrap()
            .add_number(3)
            .unwrap()
            .add_number(4)
            .unwrap()
            .add_number(5)
            .unwrap()
            .add_number(7)
            .unwrap()
            .add_number(8)
            .unwrap()
            .build()
            .unwrap();

        // Replace 8 with 0 to simulate a zero on the board
        let board_with_zero = BoardAdjuster::from(&base_board)
            .remove_number(8)
            .add_number(0)
            .build();

        assert!(
            board_with_zero.numbers().contains(&0),
            "Test setup: board should contain zero"
        );

        let children = IterativeDeepeningSolver::generate_children(&board_with_zero);

        // No child should have been produced by dividing by zero
        for (_, operation) in &children {
            if operation.operator == Operator::Divide {
                assert!(
                    operation.right != 0,
                    "Division by zero should have been skipped, but found: {} / {} = {}",
                    operation.left,
                    operation.right,
                    operation.result
                );
                assert!(
                    operation.left != 0 || operation.right != 0,
                    "Division involving zero should have been skipped: {} / {}",
                    operation.left,
                    operation.right
                );
            }
        }
    }
}
