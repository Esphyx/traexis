use super::queue::tetromino::Tetromino;

pub mod shapes;

pub struct Piece {
    pub rotation: Rotation,
    pub tetromino: Tetromino,
}

impl Piece {
    pub fn get_shape(&self) -> () {
        // shapes::SHAPES[self.tetromino as usize]
    }
}

pub struct Rotation {
    pub direction: Face,
    pub angle: Turn,
}

#[derive(strum_macros::EnumCount)]
#[repr(usize)]
pub enum Face {
    Front,
    Back,
    Right,
    Left,
    Top,
    Bottom,
}

#[derive(strum_macros::EnumCount)]
#[repr(usize)]
pub enum Turn {
    No,
    Clockwise,
    AntiClockwise,
    Half,
}
