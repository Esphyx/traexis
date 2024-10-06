use crate::tetromino::Tetromino;
use std::collections::HashMap;

pub trait Parsing {
    fn parse(input: impl Into<String>) -> Result<Self, String>
    where
        Self: Sized;
}

pub struct Queue {
    sequence: Vec<Pattern>,
    hold: Tetromino,
}

impl Parsing for Queue {
    fn parse(input: impl Into<String>) -> Result<Self, String> {
        let input = input.into();
        todo!()
    }
}

pub struct Pattern {
    multiset: HashMap<Tetromino, usize>,
    pick: usize,
}

impl Parsing for Pattern {
    fn parse(input: impl Into<String>) -> Result<Self, String> {
        let input = input.into();
        todo!()
    }
}
