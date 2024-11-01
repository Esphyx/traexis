#[derive(Debug)]
pub enum Action {
    MoveNegX,
    MovePosX,
    MoveNegZ,
    MovePosZ,
    SoftDrop,
    HardDrop,
    RotateForward,
    RotateBackward,
    RotateRight,
    RotateLeft,
    RotateClockwise,
    RotateAntiClockwise,
    SwapHold,
}

pub enum Rotation {
    Forward,
    Backward,
}
