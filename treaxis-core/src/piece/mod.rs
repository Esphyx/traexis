use super::queue::tetromino::Tetromino;

pub mod shapes;

pub struct Piece {
    pub position: [usize; 3],
    pub rotation: Rotation,
    pub tetromino: Tetromino,
}

impl Piece {
    pub fn get_shape(&self) -> [[[bool; 4]; 4]; 4] {
        shapes::SHAPES[self.tetromino as usize][self.rotation.direction as usize]
            [self.rotation.angle as usize]
    }
}

#[derive(Clone, Copy)]
pub struct Rotation {
    pub direction: Face,
    pub angle: Turn,
}

#[derive(strum_macros::EnumCount, Clone, Copy)]
#[repr(usize)]
pub enum Face {
    Front,
    Back,
    Right,
    Left,
    Top,
    Bottom,
}

#[derive(strum_macros::EnumCount, Clone, Copy)]
#[repr(usize)]
pub enum Turn {
    No,
    Clockwise,
    AntiClockwise,
    Half,
}
