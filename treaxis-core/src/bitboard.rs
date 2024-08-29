use std::fmt::Display;

#[derive(Clone, Copy, Default)]
pub struct Bitboard<const WIDTH: usize, const DEPTH: usize> {
    value: u64,
}

impl<const WIDTH: usize, const DEPTH: usize> Bitboard<WIDTH, DEPTH> {
    pub fn new() -> Self {
        Self { value: 0 }
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
            64 // throws overflow error
        }
    }
}

impl<const WIDTH: usize, const DEPTH: usize> Display for Bitboard<WIDTH, DEPTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for z in (0..DEPTH).rev() {
            for x in 0..WIDTH {
                output += if self.get(x, z) { "▓▓" } else { "░░" };
            }
            if z != 0 {
                output += "\n";
            }
        }

        f.write_str(&output)
    }
}
