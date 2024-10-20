#[derive(
    strum_macros::EnumIter,
    strum_macros::EnumString,
    strum_macros::EnumCount,
    PartialEq,
    Eq,
    Debug,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
)]
#[repr(usize)]
pub enum Tetromino {
    I,
    O,
    T,
    L,
    S,
    B,
    D,
    F,
}

impl Tetromino {
    pub fn get_color(&self) -> [f32; 3] {
        match self {
            Tetromino::I => [0., 1., 1.],
            Tetromino::O => [1., 1., 0.],
            Tetromino::T => [1., 0., 1.],
            Tetromino::L => [1., 0.5, 0.],
            Tetromino::S => [0., 1., 0.],
            Tetromino::B => [0.5, 0.15, 0.],
            Tetromino::D => [0.25, 0.25, 0.25],
            Tetromino::F => [0.7, 0.7, 0.7],
        }
    }
}

impl super::queue::Parsing for Tetromino {
    fn parse<T: Into<String>>(value: T) -> Result<Self, String> {
        let input = value.into();
        <Self as std::str::FromStr>::from_str(input.as_str())
            .map_err(|_| format!("Invalid tetromino character '{}'", input))
    }
}

impl std::fmt::Display for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

