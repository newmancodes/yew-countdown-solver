use crate::game::board::Board;
use crate::game::game::Game;
use crate::solver::solver::{Solution, Solvable, Solver};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct IterativeDeepeningSolver<'a, T> {
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
}

impl<'a> Solver<'a, Game> for IterativeDeepeningSolver<'a, Game> {
    fn solve(&self) -> Option<Solution<'a, Game>> {
        if self.initial_state.is_solved() {
            return Some(Solution::new(self.initial_state, 2));
        }

        let mut depth_limit = 1;

        while depth_limit < self.maximum_depth {
            // let mut frontier = Vec::default();
            // let mut explored = HashSet::default();

            // frontier.push();
            //
            // while let Some(candidate) = frontier.pop() {
            //     if candidate.board.contains_number(self.initial_state.target()) {
            //         return Some(Solution::new(candidate, depth_limit + 2));
            //     }
            //
            //     for child_candidate in candidate.generate_children() {
            //         if child_candidate.depth() < depth_limit
            //             && !frontier.contains(&child_candidate)
            //             && !explored.contains(&child_candidate) {
            //             frontier.push(child_candidate);
            //         }
            //     }
            // }

            depth_limit += 1;
        }

        None
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

        assert_eq!(solution.steps(), 2);
    }

    #[test]
    #[ignore = "implementation is in progress"]
    fn solvable_games_are_solved_in_the_expected_number_of_steps() {
        let games_with_expected_solution_steps = [
            (game!(12, 1, 2, 3, 4, 5, 6), 3),
            (game!(350, 1, 4, 4, 5, 6, 50), 4),
            (game!(410, 1, 3, 3, 8, 9, 50), 5),
            (game!(277, 2, 3, 3, 5, 6, 75), 6),
            (game!(831, 1, 10, 25, 50, 75, 100), 7),
        ];

        for (game, expected_solution_steps) in games_with_expected_solution_steps {
            let solver = IterativeDeepeningSolver::new(&game);

            let solution = solver.solve();

            assert!(solution.is_some(), "Expected solution for game {:?}", game);

            let solution = solution.unwrap();

            assert_eq!(
                solution.steps(),
                expected_solution_steps,
                "Wrong solution steps for {:?} expected {} received {}",
                game,
                expected_solution_steps,
                solution.steps()
            );
        }
    }
}
