use crate::game::game::Game;

pub trait GameSolver {
    fn solve(game: &Game) -> ();
}
