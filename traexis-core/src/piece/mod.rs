use std::ops::AddAssign;

use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

use super::tetromino::Tetromino;

pub mod shapes;

#[derive(Debug)]
pub struct Piece {
    pub tetromino: Tetromino,
    pub position: [i32; 3],
    pub orientation: Orientation,
}

impl Piece {
    pub fn get_shape(&self) -> u64 {
        let Orientation { direction, angle } = self.orientation;
        shapes::SHAPES[self.tetromino as usize][direction as usize][angle as usize]
    }
    pub fn contains_mino(shape: u64, u: usize, v: usize, w: usize) -> bool {
        let index = u + v * shapes::SIZE * shapes::SIZE + w * shapes::SIZE;

        shape >> index & 0x1 == 1
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Orientation {
    pub direction: Axis,
    pub angle: Turn,
}

impl Orientation {
    pub fn rotate(&self) -> Orientation {
        todo!()
    }
}

#[derive(EnumCount, Clone, Copy, Default, FromRepr, Debug)]
#[repr(usize)]
pub enum Axis {
    PosX,
    #[default]
    PosY,
    PosZ,
    NegX,
    NegY,
    NegZ,
}

impl Axis {
    pub const fn align(self, shape: u64) -> u64 {
        // relative to PosY (the default rotation)
        match self {
            Axis::PosX => shapes::rotate_bitboard(shape, Axis::NegZ),
            Axis::PosY => return shape,
            Axis::PosZ => shapes::rotate_bitboard(shape, Axis::PosX),
            Axis::NegX => shapes::rotate_bitboard(shape, Axis::PosZ),
            Axis::NegY => {
                shapes::rotate_bitboard(shapes::rotate_bitboard(shape, Axis::PosX), Axis::PosX)
            }
            Axis::NegZ => shapes::rotate_bitboard(shape, Axis::NegX),
        }
    }

    pub fn invert(self) -> Self {
        match self {
            Axis::PosX => Axis::NegX,
            Axis::PosY => Axis::NegY,
            Axis::PosZ => Axis::NegZ,
            Axis::NegX => Axis::PosX,
            Axis::NegY => Axis::PosY,
            Axis::NegZ => Axis::PosZ,
        }
    }
}

impl AddAssign<u32> for Axis {
    fn add_assign(&mut self, rhs: u32) {
        *self = Self::from_repr(((*self as usize) + rhs as usize) % Self::COUNT).unwrap();
    }
}

#[derive(EnumCount, Clone, Copy, FromRepr, Debug, Default)]
#[repr(usize)]
pub enum Turn {
    #[default]
    No,
    Clockwise,
    Half,
    AntiClockwise,
}

impl AddAssign<u32> for Turn {
    fn add_assign(&mut self, rhs: u32) {
        *self = Self::from_repr(((*self as usize) + rhs as usize) % Self::COUNT)
            .expect("Could not convert usize to Turn");
    }
}
