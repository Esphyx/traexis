pub mod action;
pub mod layer;
pub mod piece;
pub mod queue;
pub mod tetromino;

use action::Action;
use layer::Layer;
use piece::{shapes, Orientation, Piece};
use tetromino::Tetromino;

pub struct State<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> {
    pub current: Piece,
    pub playfield: [Layer<WIDTH, DEPTH>; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> State<WIDTH, HEIGHT, DEPTH> {
    pub fn process_action(&mut self, action: Action) {
        println!(
            "processing action: {:?} for current piece: {:?}",
            action, self.current
        );
        match action {
            Action::MoveNegZ => {
                self.current.position[2] += 1;
            }
            Action::MovePosZ => {
                self.current.position[2] -= 1;
            }
            Action::MoveNegX => {
                self.current.position[0] -= 1;
            }
            Action::MovePosX => {
                self.current.position[0] += 1;
            }
            Action::SoftDrop => todo!(),
            Action::HardDrop => todo!(),
            Action::RotateForward => todo!(),
            Action::RotateBackward => todo!(),
            Action::RotateRight => todo!(),
            Action::RotateLeft => todo!(),
            Action::RotateClockwise => todo!(),
            Action::RotateAntiClockwise => todo!(),
            Action::SwapHold => todo!(),
        }
    }

    pub fn clear(&mut self) {
        self.playfield = [Layer::<WIDTH, DEPTH>::default(); HEIGHT];
    }

    pub fn merge(&mut self, shape: u64, pos: [usize; 3]) -> bool {
        let [x, y, z] = pos;


        // NOT NECESSARILY OUT OF BOUNDS
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
                    direction: piece::Axis::PosY,
                    angle: piece::Turn::No,
                },
                tetromino: Tetromino::I,
            },
            playfield: [layer::Layer::<WIDTH, DEPTH>::default(); HEIGHT],
        }
    }
}
