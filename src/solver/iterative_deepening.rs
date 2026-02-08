use crate::game::board::{Board, BoardAdjuster};
use crate::game::game::Game;
use crate::solver::solver::{Instruction, Problem, Solution, Solver};
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

    fn is_solved(game: &Game) -> bool {
        game.is_solved()
    }

    fn generate_children(board: &Board) -> Vec<Board> {
        let mut children = Vec::<Board>::new();

        for i in 0..(board.numbers().len() - 1) {
            for j in i + 1..board.numbers().len() {
                // Addition
                children.push(
                    BoardAdjuster::from(board)
                        .remove_number(board.numbers()[i])
                        .remove_number(board.numbers()[j])
                        .add_number(board.numbers()[i] + board.numbers()[j])
                        .build(),
                );

                // Multiply
                children.push(
                    BoardAdjuster::from(board)
                        .remove_number(board.numbers()[i])
                        .remove_number(board.numbers()[j])
                        .add_number(board.numbers()[i] * board.numbers()[j])
                        .build(),
                );

                // Subtraction
                if board.numbers()[i] != board.numbers()[j] {
                    let mut board_adjuster = BoardAdjuster::from(board)
                        .remove_number(board.numbers()[i])
                        .remove_number(board.numbers()[j]);

                    if board.numbers()[i] > board.numbers()[j] {
                        board_adjuster =
                            board_adjuster.add_number(board.numbers()[i] - board.numbers()[j]);
                    } else {
                        board_adjuster =
                            board_adjuster.add_number(board.numbers()[j] - board.numbers()[i]);
                    }

                    children.push(board_adjuster.build());
                }

                // Division
                if board.numbers()[i] == board.numbers()[j] {
                    children.push(
                        BoardAdjuster::from(board)
                            .remove_number(board.numbers()[i])
                            .remove_number(board.numbers()[j])
                            .add_number(1)
                            .build(),
                    );
                } else if board.numbers()[i] > board.numbers()[j]
                    && board.numbers()[i] % board.numbers()[j] == 0
                {
                    children.push(
                        BoardAdjuster::from(board)
                            .remove_number(board.numbers()[i])
                            .remove_number(board.numbers()[j])
                            .add_number(board.numbers()[i] / board.numbers()[j])
                            .build(),
                    );
                } else if board.numbers()[j] % board.numbers()[i] == 0 {
                    children.push(
                        BoardAdjuster::from(board)
                            .remove_number(board.numbers()[i])
                            .remove_number(board.numbers()[j])
                            .add_number(board.numbers()[j] / board.numbers()[i])
                            .build(),
                    );
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
                    instructions.push(Instruction::new(candidate.state().clone()));
                    let mut previous_state = candidate.previous_state();
                    while let Some(visited_state) = previous_state {
                        instructions.push(Instruction::new(visited_state.state().clone()));
                        previous_state = visited_state.previous_state();
                    }
                    instructions.reverse();
                    return Some(Solution::new(self.initial_state.clone(), instructions));
                }

                for child_candidate in Self::generate_children(candidate.state()) {
                    if self.calculate_child_depth(&child_candidate) < depth_limit
                        && !explored.contains(&child_candidate)
                        && !frontier
                            .iter()
                            .any(|traversal| traversal.state() == &child_candidate)
                    {
                        frontier.push(StateTraversal::intermediate_state(
                            candidate.clone(),
                            child_candidate,
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! game {
        ($target:expr, $($num:expr),+ $(,)?) => {{
            let mut builder = Board::builder();
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
}
