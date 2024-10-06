use std::fmt::Display;

pub struct Bitboard<const W: usize, const D: usize> {
    pub value: u64,
}

impl<const W: usize, const D: usize> Bitboard<W, D> {
    pub fn new() -> Self {
        if W > 8 || D > 8 {
            panic!("Generics must be between 0-7");
        }
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
    pub fn index(x: usize, z: usize) -> usize {
        if x < W && z < D {
            return x + z * W;
        }

        panic!("{} {} are out of bounds for {} {}", x, z, W, D);
    }
}

impl<const W: usize, const D: usize> Display for Bitboard<W, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::*;
        let mut str = String::new();

        for z in 0..D {
            for x in 0..W {
                const BLOCK: &'static str = "██";
                str.push_str(&if self.get(x, z) {
                    BLOCK.white()
                } else {
                    BLOCK.black()
                }.to_string());
            }
            str.push_str("\n");
        }

        f.write_str(&str)
    }
}
