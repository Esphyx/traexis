pub struct Piece {
    pub rotation: Rotation,
}

pub struct Rotation {
    pub face: Face,
    pub turn: Turn,
}

pub enum Face {
    Front,
    Back,
    Right,
    Left,
    Top,
    Bottom,
}

pub enum Turn {
    No,
    Clockwise,
    AntiClockwise,
    Half,
}
