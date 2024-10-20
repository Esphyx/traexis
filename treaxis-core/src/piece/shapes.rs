use super::Axis;

pub const SIZE: usize = 4;
// pub const SHAPES: [[[u64; Turn::COUNT]; Axis::COUNT]; Tetromino::COUNT] = [

// ];

pub const SHAPES: [u64; 1] = [rotate_bitboard(
    0b0000_0000_0000_0000_0000_0000_0000_0000_0001_0001_0001_0001_0000_0000_0011_0110,
    Axis::PosX,
)];

// pub const SHAPES: [u64; 1] =
// [0b0000_0000_0000_0000_0000_0000_0000_0000_0001_0001_0001_0001_0000_0000_0011_0110];

const fn rotate_bitboard(bb: u64, axis: Axis) -> u64 {
    let mut result = 0;

    #[inline]
    const fn index(x: usize, y: usize, z: usize) -> usize {
        x + z * 4 + y * 4 * 4
    }

    let mut y = 0;
    while y < 4 {
        let mut z = 0;
        while z < 4 {
            let mut x = 0;
            while x < 4 {
                let is_set = bb >> index(x, y, z) & 1;
                let i = match axis {
                    Axis::NegX => todo!(),
                    Axis::PosX => index(x, SIZE - z - 1, y),
                    Axis::PosY => todo!(),
                    Axis::PosZ => todo!(),
                    Axis::NegY => todo!(),
                    Axis::NegZ => todo!(),
                };
                result |= is_set << i;

                x += 1;
            }
            z += 1;
        }
        y += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{rotate_bitboard, Axis};

    #[test]
    pub fn rotate() {
        //                  :               |                   |                   |
        let bb = 0b0000000000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0011_0010;
        //              0000000000000010000000000000001100000000000000000000000000000000

        let rotated = rotate_bitboard(bb, Axis::PosX);
        println!("{:064b}", rotated);
    }
}
