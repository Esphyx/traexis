#![allow(unused)]

#[inline]
pub fn add<T>(first: [T; 3], second: [T; 3]) -> [T; 3]
where
    T: std::ops::Add<Output = T> + Copy,
{
    [
        first[0] + second[0],
        first[1] + second[1],
        first[2] + second[2],
    ]
}

#[inline]
pub fn subtract<T>(first: [T; 3], second: [T; 3]) -> [T; 3]
where
    T: std::ops::Sub<Output = T> + Copy,
{
    [
        first[0] - second[0],
        first[1] - second[1],
        first[2] - second[2],
    ]
}

#[inline]
pub fn negate<T>(vector: [T; 3]) -> [T; 3]
where
    T: std::ops::Neg<Output = T> + Copy,
{
    [-vector[0], -vector[1], -vector[2]]
}

#[inline]
pub fn cross<T>(first: [T; 3], second: [T; 3]) -> [T; 3]
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    [
        first[1] * second[2] - first[2] * second[1],
        first[2] * second[0] - first[0] * second[2],
        first[0] * second[1] - first[1] * second[0],
    ]
}

#[inline]
pub fn magnitude(vector: [f32; 3]) -> f32 {
    (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt()
}

#[inline]
pub fn normalize(vector: [f32; 3]) -> [f32; 3] {
    let mag = magnitude(vector);
    if mag == 0.0 {
        return [0.0, 0.0, 0.0];
    }
    [vector[0] / mag, vector[1] / mag, vector[2] / mag]
}

#[inline]
pub fn rotate_x(vector: [f32; 3], angle: f32) -> [f32; 3] {
    [
        vector[0],
        angle.cos() * vector[1] - angle.sin() * vector[2],
        angle.sin() * vector[1] + angle.cos() * vector[2],
    ]
}

#[inline]
pub fn rotate_y(vector: [f32; 3], angle: f32) -> [f32; 3] {
    [
        angle.cos() * vector[0] + angle.sin() * vector[2],
        vector[1],
        angle.cos() * vector[2] - angle.sin() * vector[0],
    ]
}

#[inline]
pub fn rotate_z(vector: [f32; 3], angle: f32) -> [f32; 3] {
    [
        angle.cos() * vector[0] - angle.sin() * vector[1],
        angle.sin() * vector[0] - angle.cos() * vector[1],
        vector[2],
    ]
}

#[inline]
pub fn dot<T>(first: [T; 3], second: [T; 3]) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    first[0] * second[0] + first[1] * second[1] + first[2] * second[2]
}

#[inline]
pub fn scale<T>(vector: [T; 3], multiplier: T) -> [T; 3]
where
    T: std::ops::Mul<Output = T> + Copy,
{
    [
        vector[0] * multiplier,
        vector[1] * multiplier,
        vector[2] * multiplier,
    ]
}

#[inline]
pub fn rodrigues(vector: [f32; 3], axis: [f32; 3], angle: f32) -> [f32; 3] {
    add(
        add(
            scale(vector, angle.cos()),
            scale(cross(axis, vector), angle.sin()),
        ),
        scale(axis, dot(axis, vector) * (1.0 - angle.cos())),
    )
}

#[cfg(test)]
mod tests {
    use crate::client::linear_algebra::{add, subtract};

    #[test]
    pub fn addition() {
        assert_eq!(add([2.4, 4.6, 4.7], [-6.6, 2.4, 6.3]), [-4.2, 7.0, 11.0])
    }

    #[test]
    pub fn subtraction() {
        assert_eq!(
            subtract([0.6, -2.8, -2.6], [4.1, 3.2, 6.8]),
            [-3.5, -6.0, -9.4]
        );
    }
}
