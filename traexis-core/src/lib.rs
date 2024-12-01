pub mod action;
pub mod layer;
pub mod piece;
pub mod queue;
pub mod tetromino;

use action::Action;
use layer::Layer;
use piece::{shapes, Orientation, Piece};
use queue::{Parsing, Queue};
use tetromino::Tetromino;

pub struct State<const WIDTH: usize, const HEIGHT: usize, const DEPTH: usize> {
    pub queue: Queue,
    pub current_piece: Piece,
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
                if !self.try_direction([0, -1, 0]) {
                    self.place_piece();
                }
            }
            Action::HardDrop => while self.try_direction([0, -1, 0]) {},
            Action::RotatePosX => todo!(),
            Action::RotateNegX => todo!(),
            Action::RotatePosY => todo!(),
            Action::RotateNegY => todo!(),
            Action::RotatePosZ => todo!(),
            Action::RotateNegZ => todo!(),
            Action::SwapHold => self.swap_hold(),
        }

        println!("Current piece location: {:?}", self.current_piece.position);
        println!("Queue: {}", self.queue);
    }

    pub fn swap_hold(&mut self) {
        if self.queue.can_swap {
            if let Some(hold_type) = self.queue.hold {
                self.queue.hold = Some(self.current_piece.tetromino);
                self.current_piece = Self::piece_from_tetromino(hold_type);
            } else {
                self.queue.hold = Some(self.current_piece.tetromino);
                self.spawn();
            }
        }
    }

    pub fn clear(&mut self) {
        self.playfield = [Layer::<WIDTH, DEPTH>::default(); HEIGHT];
    }

    pub fn try_direction(&mut self, offset: [i32; 3]) -> bool {
        let mut pos = self.current_piece.position;
        pos = [pos[0] + offset[0], pos[1] + offset[1], pos[2] + offset[2]];
        let does_fit = self.fits(self.current_piece.get_shape(), pos, false);
        if does_fit {
            self.current_piece.position = pos;
        }
        does_fit
    }

    pub fn place_piece(&mut self) {
        if !self.fits(
            self.current_piece.get_shape(),
            self.current_piece.position,
            true,
        ) {
            panic!("could not place piece!");
        }
        self.clear_full_layers();
        self.spawn();
    }

    pub fn clear_full_layers(&mut self) {
        let mut l = 0;
        let mut cleared_layer_count = 0;
        while l < HEIGHT - cleared_layer_count {
            let layer = &mut self.playfield[l];

            if layer.is_full() {
                println!("Layer {l} is full!");
                cleared_layer_count += 1;
                for i in l..HEIGHT - cleared_layer_count {
                    self.playfield[i] = self.playfield[i + 1];
                }
            }

            l += 1;
        }

        for i in 0..cleared_layer_count {
            self.playfield[HEIGHT - i - 1].clear();
        }
    }

    pub fn spawn(&mut self) {
        self.current_piece =
            Self::piece_from_tetromino(self.queue.next().expect("There is no next tetromino!"));
    }

    pub fn piece_from_tetromino(tetromino: Tetromino) -> Piece {
        let x = (WIDTH - shapes::SIZE) / 2;
        let z = (DEPTH - shapes::SIZE) / 2;

        let position = [x as i32, HEIGHT as i32, z as i32];

        Piece {
            position,
            orientation: Orientation::default(),
            tetromino,
        }
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
            if i as i32 + y >= HEIGHT as i32 {
                continue;
            }
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
        // let mut queue = Queue::parse("[IIII]p4").unwrap();
        let mut queue = Queue::default();
        let current_piece =
            Self::piece_from_tetromino(queue.next().expect("There isn't a next tetromino!"));
        let playfield = [layer::Layer::<WIDTH, DEPTH>::default(); HEIGHT];

        Self {
            queue,
            current_piece,
            playfield,
        }
    }
}
