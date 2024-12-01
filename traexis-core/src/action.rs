#[derive(Debug)]
pub enum Action {
    MoveNegX,
    MovePosX,
    MoveNegZ,
    MovePosZ,
    SoftDrop,
    HardDrop,
    RotatePosX,
    RotateNegX,
    RotatePosY,
    RotateNegY,
    RotatePosZ,
    RotateNegZ,
    SwapHold,
}

pub enum Rotation {
    Forward,
    Backward,
}
