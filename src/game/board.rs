use rand::seq::SliceRandom;
use rand::Rng;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Board {
    numbers: Vec<u32>,
}

impl Board {
    const SMALL_NUMBERS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const LARGE_NUMBERS: &[u8] = &[25, 50, 75, 100];

    pub fn numbers(&self) -> &[u32] {
        &self.numbers
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();

        let large_number_count = rng.random_range(0..=Board::LARGE_NUMBERS.len() as u8);
        let small_number_count = 6 - large_number_count;

        Self::random_with_number_mix_specified(small_number_count, large_number_count).unwrap()
    }

    pub fn random_with_number_mix_specified(
        small_number_count: u8,
        large_number_count: u8,
    ) -> Result<Self, BoardError> {
        if small_number_count + large_number_count < 6 {
            return Err(BoardError::UnderpopulatedBoard);
        }

        if small_number_count + large_number_count > 6 {
            return Err(BoardError::OverpopulatedBoard);
        }

        let mut rng = rand::rng();

        let mut board_builder = BoardBuilder::new();

        let mut available_small_numbers =
            [Board::SMALL_NUMBERS.to_vec(), Board::SMALL_NUMBERS.to_vec()].concat();
        available_small_numbers.shuffle(&mut rng);

        for _ in 0..small_number_count {
            let index = rng.random_range(0..available_small_numbers.len() as u8) as usize;
            let selected_small_number = available_small_numbers.remove(index);

            board_builder = board_builder.add_number(selected_small_number)?;
        }

        let mut available_large_numbers = Board::LARGE_NUMBERS.to_vec();

        for _ in 0..large_number_count {
            let index = rng.random_range(0..available_large_numbers.len() as u8) as usize;
            let selected_large_number = available_large_numbers.remove(index);

            board_builder = board_builder.add_number(selected_large_number)?;
        }

        board_builder.build()
    }
}

#[derive(Debug)]
pub struct BoardBuilder {
    numbers: Vec<u8>,
}

impl Default for BoardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardBuilder {
    pub fn new() -> Self {
        BoardBuilder {
            numbers: Vec::with_capacity(6),
        }
    }

    pub fn add_number(mut self, number: u8) -> Result<Self, BoardError> {
        if !self.is_valid_number(number) {
            return Err(BoardError::InvalidNumber(number));
        }

        if self.is_small_number(number) {
            if self.count_number_usage(number) >= 2 {
                return Err(BoardError::SmallNumberUsedTooManyTimes(number));
            }
        } else if self.count_number_usage(number) > 0 {
            return Err(BoardError::LargeNumberAlreadyUsed(number));
        }

        if self.numbers.len() >= 6 {
            return Err(BoardError::OverpopulatedBoard);
        }

        self.numbers.push(number);

        Ok(self)
    }

    pub fn build(mut self) -> Result<Board, BoardError> {
        if self.numbers.len() < 6 {
            return Err(BoardError::UnderpopulatedBoard);
        }

        self.numbers.sort_unstable();

        Ok(Board {
            numbers: self
                .numbers
                .into_iter()
                .map(|n| n as u32)
                .collect::<Vec<_>>(),
        })
    }

    fn is_valid_number(&self, number: u8) -> bool {
        const ALLOWED: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 25, 50, 75, 100];
        ALLOWED.contains(&number)
    }

    fn is_small_number(&self, number: u8) -> bool {
        Board::SMALL_NUMBERS.contains(&number)
    }

    fn count_number_usage(&self, number: u8) -> usize {
        self.numbers.iter().filter(|&&n| n == number).count()
    }
}

#[derive(Error, Debug)]
pub enum BoardError {
    #[error("Invalid number: {0}. Only 1-10, 25, 50, 75, and 100 are allowed")]
    InvalidNumber(u8),

    #[error("Large number {0} can only be used once")]
    LargeNumberAlreadyUsed(u8),

    #[error("Number {0} can only be used twice")]
    SmallNumberUsedTooManyTimes(u8),

    #[error("Board is underpopulated")]
    UnderpopulatedBoard,

    #[error("Board is overpopulated")]
    OverpopulatedBoard,
}

#[derive(Debug)]
pub struct BoardAdjuster {
    numbers: Vec<u32>,
}

impl BoardAdjuster {
    pub fn from(board: &Board) -> Self {
        BoardAdjuster {
            numbers: board.numbers().to_vec(),
        }
    }

    pub fn remove_number(self, number: u32) -> Self {
        let mut numbers = self.numbers;
        if let Some(pos) = numbers.iter().position(|&n| n == number) {
            numbers.remove(pos);
        }

        BoardAdjuster { numbers }
    }

    pub fn add_number(self, number: u32) -> Self {
        BoardAdjuster {
            numbers: [self.numbers, vec![number]].concat(),
        }
    }

    pub fn build(self) -> Board {
        let mut numbers = self.numbers;
        numbers.sort_unstable();
        Board { numbers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_which_do_not_exist_on_the_game_cards_are_not_allowed() {
        let disallowed_numbers: Vec<u8> = (0..=255)
            .filter(|&n| !((1..=10).contains(&n) || matches!(n, 25 | 50 | 75 | 100)))
            .collect();

        for number in disallowed_numbers {
            let board = BoardBuilder::new().add_number(number);

            assert!(board.is_err());

            let err = board.err().unwrap();

            assert!(matches!(err, BoardError::InvalidNumber(_)));

            assert_eq!(
                format!("{}", err),
                format!(
                    "Invalid number: {}. Only 1-10, 25, 50, 75, and 100 are allowed",
                    number
                )
            );
        }
    }

    #[test]
    fn small_numbers_can_be_reused_twice() -> Result<(), BoardError> {
        let small_numbers: Vec<u8> = (1..=10).collect();

        for number in small_numbers {
            let board = BoardBuilder::new()
                .add_number(number)?
                .add_number(number)?
                .add_number(number);

            assert!(board.is_err(), "Expected error for small number {}", number);

            let err = board.err().unwrap();

            assert!(
                matches!(err, BoardError::SmallNumberUsedTooManyTimes(_)),
                "Wrong error type for number {}: {:?}",
                number,
                err
            );

            assert_eq!(
                format!("{}", err),
                format!("Number {} can only be used twice", number),
                "Wrong error message for number {}",
                number
            );
        }

        Ok(())
    }

    #[test]
    fn large_numbers_can_be_used_once() -> Result<(), BoardError> {
        let large_numbers = [25, 50, 75, 100];

        for number in large_numbers {
            let board = BoardBuilder::new().add_number(number)?.add_number(number);

            assert!(board.is_err(), "Expected error for large number {}", number);

            let err = board.err().unwrap();

            assert!(
                matches!(err, BoardError::LargeNumberAlreadyUsed(_)),
                "Wrong error type for number {}: {:?}",
                number,
                err
            );

            assert_eq!(
                format!("{}", err),
                format!("Large number {} can only be used once", number),
                "Wrong error message for number {}",
                number
            );
        }

        Ok(())
    }

    #[test]
    fn underpopulated_board_fails() -> Result<(), BoardError> {
        let board = BoardBuilder::new()
            .add_number(1)?
            .add_number(2)?
            .add_number(3)?
            .add_number(4)?
            .add_number(5)?
            .build();

        assert!(board.is_err(), "Expected error for underpopulated board");

        let err = board.err().unwrap();

        assert!(
            matches!(err, BoardError::UnderpopulatedBoard),
            "Wrong error type for underpopulated board"
        );

        assert_eq!(
            format!("{}", err),
            "Board is underpopulated",
            "Wrong error message for underpopulated board"
        );

        Ok(())
    }

    #[test]
    fn adding_a_seventh_number_fails() -> Result<(), BoardError> {
        let board = BoardBuilder::new()
            .add_number(1)?
            .add_number(2)?
            .add_number(3)?
            .add_number(4)?
            .add_number(5)?
            .add_number(6)?
            .add_number(7);

        assert!(board.is_err(), "Expected error for overpopulated board");

        let err = board.err().unwrap();

        assert!(
            matches!(err, BoardError::OverpopulatedBoard),
            "Wrong error type for overpopulated board"
        );

        assert_eq!(
            format!("{}", err),
            "Board is overpopulated",
            "Wrong error message for overpopulated board"
        );

        Ok(())
    }

    #[test]
    fn valid_board_can_be_built() -> Result<(), BoardError> {
        let board = BoardBuilder::new()
            .add_number(2)?
            .add_number(3)?
            .add_number(3)?
            .add_number(5)?
            .add_number(6)?
            .add_number(75)?
            .build()?;

        assert_eq!(board.numbers(), [2, 3, 3, 5, 6, 75]);

        Ok(())
    }

    #[test]
    fn valid_boards_are_their_numbers_sorted_in_ascending_order() -> Result<(), BoardError> {
        let board = BoardBuilder::new()
            .add_number(6)?
            .add_number(75)?
            .add_number(2)?
            .add_number(3)?
            .add_number(3)?
            .add_number(5)?
            .build()?;

        assert_eq!(board.numbers(), [2, 3, 3, 5, 6, 75]);

        Ok(())
    }

    #[test]
    fn random_boards_are_valid() {
        let board = Board::random();

        assert_eq!(board.numbers().len(), 6);
    }

    #[test]
    fn underpopulated_number_mix_specified_random_boards_fails() -> Result<(), BoardError> {
        let board = Board::random_with_number_mix_specified(2, 3);

        assert!(board.is_err(), "Expected error for underpopulated board");

        let err = board.err().unwrap();

        assert!(
            matches!(err, BoardError::UnderpopulatedBoard),
            "Wrong error type for underpopulated board"
        );

        assert_eq!(
            format!("{}", err),
            "Board is underpopulated",
            "Wrong error message for underpopulated board"
        );

        Ok(())
    }

    #[test]
    fn overpopulated_number_mix_specified_random_boards_fails() -> Result<(), BoardError> {
        let board = Board::random_with_number_mix_specified(3, 4);

        assert!(board.is_err(), "Expected error for overpopulated board");

        let err = board.err().unwrap();

        assert!(
            matches!(err, BoardError::OverpopulatedBoard),
            "Wrong error type for overpopulated board"
        );

        assert_eq!(
            format!("{}", err),
            "Board is overpopulated",
            "Wrong error message for overpopulated board"
        );

        Ok(())
    }

    #[test]
    fn number_mix_specified_random_boards_are_valid() {
        let number_mix_specifications = [(6, 0), (5, 1), (4, 2), (3, 3), (2, 4)];

        for (small_number_count, large_number_count) in number_mix_specifications {
            let board =
                Board::random_with_number_mix_specified(small_number_count, large_number_count)
                    .unwrap();

            let actual_small_number_count = board
                .numbers()
                .iter()
                .filter(|&&n| Board::SMALL_NUMBERS.contains(&(n as u8)))
                .count();

            let actual_large_number_count = board
                .numbers()
                .iter()
                .filter(|&&n| Board::LARGE_NUMBERS.contains(&(n as u8)))
                .count();

            assert_eq!(
                actual_small_number_count, small_number_count as usize,
                "Expected {} small numbers, got {}",
                small_number_count, actual_small_number_count
            );

            assert_eq!(
                actual_large_number_count, large_number_count as usize,
                "Expected {} large numbers, got {}",
                large_number_count, actual_large_number_count
            );
        }
    }
}
