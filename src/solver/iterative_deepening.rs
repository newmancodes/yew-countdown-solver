use crate::game::board::Board;
use crate::game::game::Game;
use crate::solver::solver::{Solver, Solution};

#[derive(Debug)]
struct IterativeDeepeningSolver<'a, T> {
    initial_state: &'a T,
}

impl<'a> IterativeDeepeningSolver<'a, Game> {
    pub fn new(game: &'a Game) -> Self {
        Self {
            initial_state: game,
        }
    }
}

impl<'a> Solver<'a, Game> for IterativeDeepeningSolver<'a, Game> {
    fn solve(&self) -> Option<Solution<'a, Game>> {
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

        assert!(solution.is_some());

        let solution = solution.unwrap();

        assert_eq!(solution.steps(), 2);
    }

    #[test]
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

            assert!(solution.is_some());

            let solution = solution.unwrap();

            assert_eq!(solution.steps(), expected_solution_steps);
        }
    }
}