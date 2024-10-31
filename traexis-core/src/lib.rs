pub mod action;
pub mod bitboard;
pub mod piece;
pub mod queue;
pub mod tetromino;

use action::Action;
use bitboard::Bitboard;
use piece::{shapes, Orientation, Piece};
use tetromino::Tetromino;

pub struct State<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> {
    pub current: Piece,
    pub playfield: [Bitboard<WIDTH, DEPTH>; HEIGHT],
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

    pub fn clear(&mut self) {
        self.playfield = [Bitboard::<WIDTH, DEPTH>::default(); HEIGHT];
    }

    pub fn merge(&mut self, shape: u64, pos: (usize, usize, usize)) -> bool {
        let (x, y, z) = pos;

        if x + shapes::SIZE > WIDTH || y + shapes::SIZE > HEIGHT || z + shapes::SIZE > DEPTH {
            println!(
                "Out of bounds! x: {}, y: {}, z: {} width: {}, height: {}, depth: {}",
                x, y, z, WIDTH, HEIGHT, DEPTH
            );
            return false;
        }

        let layers = self.shape_to_layers(shape, x, z);

        for (i, &layer) in layers.iter().enumerate() {
            let bb = &mut self.playfield[i + y];
            if layer & bb.value != 0 {
                return false;
            }
            bb.value |= layer;
        }

        false
    }

    pub fn shape_to_layers(&self, mut shape: u64, x: usize, z: usize) -> [u64; 4] {
        // TODO: make a general method for WIDTH | HEIGHT < 8
        let mut results = [0; 4];
        for layer_index in 0..4 {
            let mut result = 0;
            for i in 0..4 {
                let chunk = (shape & 0xf) as u64;
                result |= chunk << (i * WIDTH);
                shape >>= 4;
            }
            results[layer_index] = result << x + z * WIDTH;
        }
        results
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
                    direction: piece::Axis::PosX,
                    angle: piece::Turn::No,
                },
                tetromino: Tetromino::I,
            },
            playfield: [bitboard::Bitboard::<WIDTH, DEPTH>::default(); HEIGHT],
        }
    }
}
