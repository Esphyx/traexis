pub mod bitboard;
pub mod piece;
pub mod queue;

pub struct State<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> {
    pub playfield: [bitboard::Bitboard<WIDTH, DEPTH>; HEIGHT],
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
