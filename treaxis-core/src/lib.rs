use action::Action;
use piece::{Orientation, Piece};

pub mod action;
pub mod bitboard;
pub mod piece;
pub mod queue;

pub struct State<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> {
    pub current: Piece,
    pub playfield: [bitboard::Bitboard<WIDTH, DEPTH>; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> State<WIDTH, HEIGHT, DEPTH> {
    pub fn process_action(&mut self, action: Action) {
        match action {
            Action::MoveForward | Action::MoveBackward | Action::MoveLeft | Action::MoveRight => {
                todo!()
            }
            Action::SoftDrop => todo!(),
            Action::HardDrop => todo!(),
            Action::RotateForward => todo!(),
            Action::RotateBackward => todo!(),
            Action::RotateRight => todo!(),
            Action::RotateLeft => todo!(),
            Action::RotateClockwise => todo!(),
            Action::RotateCounterClockwise => todo!(),
            Action::SwapHold => todo!(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> Default
    for State<WIDTH, HEIGHT, DEPTH>
{
    fn default() -> Self {
        Self {
            current: Piece {
                position: [0, 0, 0],
                orientation: Orientation {
                    direction: piece::Axis::NegZ,
                    angle: piece::Turn::No,
                },
                tetromino: queue::tetromino::Tetromino::I,
            },
            playfield: [bitboard::Bitboard::<WIDTH, DEPTH>::default(); HEIGHT],
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::queue::{pattern::*, tetromino::*, Parsing, Queue};

    use strum::IntoEnumIterator;

    #[test]
    pub fn pattern() {
        assert_eq!(
            Pattern::parse("*").unwrap(),
            Pattern {
                multiset: Tetromino::iter().map(|tetromino| (tetromino, 1)).collect(),
                amount: 1
            }
        );
        assert!(Pattern::parse("[OBFS]p9").is_err());
        assert_eq!(
            Pattern::parse("[BLIB]").unwrap(),
            Pattern {
                multiset: vec![(Tetromino::I, 1), (Tetromino::B, 2), (Tetromino::L, 1)]
                    .iter()
                    .cloned()
                    .collect(),
                amount: 1
            }
        );
    }
    #[test]
    pub fn queue() {
        let q = Queue {
            sequence: vec![
                Pattern {
                    multiset: vec![(Tetromino::B, 1), (Tetromino::F, 1)]
                        .iter()
                        .cloned()
                        .collect(),
                    amount: 2,
                },
                Pattern {
                    multiset: vec![(Tetromino::I, 1), (Tetromino::T, 1), (Tetromino::O, 1)]
                        .iter()
                        .cloned()
                        .collect(),
                    amount: 2,
                },
            ],
            hold: Some(Tetromino::L),
        };

        assert_eq!(q.to_string(), "L:[BF]p2[ITO]p2");
        assert_eq!(
            Queue::parse("T:*[DISD]p4").unwrap(),
            Queue {
                sequence: vec![
                    Pattern {
                        multiset: Tetromino::iter().map(|tetromino| (tetromino, 1)).collect(),
                        amount: 1
                    },
                    Pattern {
                        multiset: vec![(Tetromino::D, 2), (Tetromino::I, 1), (Tetromino::S, 1)]
                            .iter()
                            .cloned()
                            .collect(),
                        amount: 4,
                    }
                ],
                hold: Some(Tetromino::T)
            }
        );
    }
}
