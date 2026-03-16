use crate::game::board::Board;
use crate::solver::traits::Problem;
use rand::Rng;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    board: Board,
    target: u32,
}

impl Game {
    pub fn new(board: Board, target: u32) -> Result<Self, GameError> {
        if !(1..=999).contains(&target) {
            return Err(GameError::InvalidTarget(target));
        }

        Ok(Game { board, target })
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn target(&self) -> u32 {
        self.target
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut rng = rand::rng();
        Self::new(Board::random(), rng.random_range(1..=999)).unwrap()
    }
}

impl Problem for Game {
    fn is_solved(&self) -> bool {
        self.board.numbers().contains(&self.target)
    }
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid target: {0}. Only 1-999 is allowed")]
    InvalidTarget(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_must_be_between_1_and_999_inclusive() -> Result<(), GameError> {
        let targets: Vec<u32> = [vec![0u32], (1000u32..=1010u32).collect()].concat();

        for target in targets {
            let board = Board::random();
            let game = Game::new(board, target);

            assert!(game.is_err());

            let err = game.err().unwrap();

            assert!(
                matches!(err, GameError::InvalidTarget(_)),
                "Wrong error type for invalid target {}: {:?}",
                target,
                err
            );

            assert_eq!(
                format!("{}", err),
                format!("Invalid target: {}. Only 1-999 is allowed", target),
                "Wrong error message for invalid target {}",
                target
            );
        }

        Ok(())
    }

    #[test]
    fn default_game_is_valid() {
        let game = Game::default();

        assert_eq!(game.board().numbers().len(), 6);
        assert!(game.target() >= 1 && game.target() < 1000);
    }
}
