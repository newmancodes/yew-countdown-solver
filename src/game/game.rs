use crate::game::board::Board;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Game {
    board: Board,
    target: u16,
}

impl Game {
    fn new(board: Board, target: u16) -> Result<Self, GameError> {
        if target < 1 || target > 1000 {
            return Err(GameError::InvalidTarget(target));
        }

        Ok(Game { board, target })
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn target(&self) -> u16 {
        self.target
    }
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid target: {0}. Only 1-1000 is allowed")]
    InvalidTarget(u16),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_must_be_between_1_and_1000() -> Result<(), GameError> {
        let targets: Vec<u16> = [vec![0u16], (1001u16..=u16::MAX).collect()].concat();

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
                format!("Invalid target: {}. Only 1-1000 is allowed", target),
                "Wrong error message for invalid target {}",
                target
            );
        }

        Ok(())
    }
}
