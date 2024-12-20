use core::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Default)]
pub struct Layer<const WIDTH: usize = 8, const DEPTH: usize = 8> {
    pub value: u64,
}

impl<const WIDTH: usize, const DEPTH: usize> Layer<WIDTH, DEPTH> {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.value = 0;
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.value == !0
    }

    #[inline]
    pub fn set(&mut self, x: usize, z: usize) {
        self.value |= 1u64 << Self::index(x, z);
    }

    #[inline]
    pub fn get(&self, x: usize, z: usize) -> bool {
        self.value >> Self::index(x, z) & 1u64 == 1
    }

    #[inline]
    fn index(x: usize, z: usize) -> usize {
        if x < WIDTH {
            x + z * WIDTH
        } else {
            panic!("x = {} is not within the width = {}!", x, WIDTH);
        }
    }
}

impl<const WIDTH: usize, const DEPTH: usize> From<u64> for Layer<WIDTH, DEPTH> {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl<const WIDTH: usize, const DEPTH: usize> Display for Layer<WIDTH, DEPTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("\n");

        for z in (0..DEPTH).rev() {
            for x in 0..WIDTH {
                output += if self.get(x, z) { "▓▓" } else { "░░" };
            }
            output += "\n";
        }

        f.write_str(&output)
    }
}

impl<const WIDTH: usize, const DEPTH: usize> Debug for Layer<WIDTH, DEPTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
