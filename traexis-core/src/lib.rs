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
        match action {
            Action::MoveNegZ => {
                self.try_direction([0, 0, -1]);
            }
            Action::MovePosZ => {
                self.try_direction([0, 0, 1]);
            }
            Action::MoveNegX => {
                self.try_direction([-1, 0, 0]);
            }
            Action::MovePosX => {
                self.try_direction([1, 0, 0]);
            }
            Action::SoftDrop => {
                self.try_direction([0, -1, 0]);
            }
            Action::HardDrop => while self.try_direction([0, -1, 0]) {},
            Action::RotateForward => todo!(),
            Action::RotateBackward => todo!(),
            Action::RotateRight => todo!(),
            Action::RotateLeft => todo!(),
            Action::RotateClockwise => todo!(),
            Action::RotateAntiClockwise => todo!(),
            Action::SwapHold => todo!(),
        }

        println!(
            "processing action: {:?} for current piece at: {:?}",
            action, self.current.position
        );
    }

    pub fn clear(&mut self) {
        self.playfield = [Layer::<WIDTH, DEPTH>::default(); HEIGHT];
    }

    pub fn try_direction(&mut self, offset: [i32; 3]) -> bool {
        let mut pos = self.current.position;
        pos = [pos[0] + offset[0], pos[1] + offset[1], pos[2] + offset[2]];
        let fits = self.fits(self.current.get_shape(), pos, false);
        if fits {
            self.current.position = pos;
        }
        fits
    }

    pub fn place_piece(&mut self) {
        
    }

    pub fn fits(&mut self, mut shape: u64, pos: [i32; 3], merge: bool) -> bool {
        let [x, y, z] = pos;

        let mut layers = [0; 4];

        for y in 0..4 {
            let mut layer = 0;

            for z in 0..4 {
                let chunk = (shape & 0xf) as u64;
                layer |= chunk << (z * WIDTH);
                shape >>= 4;
            }

            layers[y] = match Self::shift_layer(layer, x, z) {
                Some(l) => l,
                None => {
                    return false;
                }
            };
        }

        for (i, &layer) in layers.iter().enumerate() {
            if i as i32 + y < 0 {
                if layer != 0 {
                    return false;
                }
                continue;
            }
            let bb = &mut self.playfield[(i as i32 + y) as usize];
            if layer & bb.value != 0 {
                return false;
            }
            if merge {
                bb.value |= layer;
            }
        }

        true
    }

    fn shift_layer(mut layer: u64, x: i32, z: i32) -> Option<u64> {
        let index = x + WIDTH as i32 * z;

        let x_border: u64 = 0x0101010101010101;

        let mut x_mask = 0;
        for i in 0..x.rem_euclid(WIDTH as i32) {
            x_mask |= x_border << i;
        }
        if x < 0 {
            x_mask = !x_mask;
        }

        let z_border: u64 = 0xff;
        let mut z_mask = 0;
        for i in 0..z.rem_euclid(DEPTH as i32) {
            z_mask |= z_border << i * WIDTH as i32;
        }
        if z < 0 {
            z_mask = !z_mask;
        }

        let mask = x_mask | z_mask;

        layer = if index < 0 {
            layer.rotate_right(index.abs() as u32)
        } else {
            layer.rotate_left(index as u32)
        };

        (layer & mask == 0).then_some(layer)
    }
}

#[cfg(test)]
mod tests {
    use crate::{layer::Layer, State};

    #[test]
    pub fn sl_test() {
        let layer =
            0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0110_0000_0011_0000_0000;
        println!("layer: \n{}", Layer::<8, 8>::from(layer));
        let x_offset = 5;
        let z_offset = 3;
        let out = State::<8, 20, 8>::shift_layer(layer, x_offset, z_offset)
            .map(|l| Layer::<8, 8>::from(l));
        println!("{:?}", out);
    }
    #[test]
    pub fn shifting() {
        let a = 1i32;
        let i = 2 as u32;
        println!("{:?}", a.overflowing_shl(i));
    }

    #[test]
    pub fn modulo() {
        let a: i32 = 5;
        let m: i32 = 4;
        let result = a.rem_euclid(m);
        println!("{}", result);
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
                tetromino: Tetromino::S,
            },
            playfield: [layer::Layer::<WIDTH, DEPTH>::default(); HEIGHT],
        }
    }
}
