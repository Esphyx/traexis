use super::tetromino::Tetromino;

pub mod shapes;

pub struct Piece {
    pub position: [usize; 3],
    pub orientation: Orientation,
    pub tetromino: Tetromino,
}

impl Piece {
    pub fn get_shape(&self) -> u64 {
        shapes::SHAPES[0]
    }
}

#[derive(Clone, Copy)]
pub struct Orientation {
    pub direction: Axis,
    pub angle: Turn,
}

impl Orientation {
    pub fn rotate(&self) -> Orientation {
        todo!()
    }
}

#[derive(strum_macros::EnumCount, Clone, Copy, Default)]
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

#[derive(strum_macros::EnumCount, Clone, Copy)]
#[repr(usize)]
pub enum Turn {
    No,
    Clockwise,
    AntiClockwise,
    Half,
}
