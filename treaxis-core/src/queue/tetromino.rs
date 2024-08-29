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

impl super::Parsing for Tetromino {
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
