#![allow(unused)]

#[inline]
pub fn add(first: [f32; 3], second: [f32; 3]) -> [f32; 3] {
    [
        first[0] + second[0],
        first[1] + second[1],
        first[2] + second[2],
    ]
}

#[inline]
pub fn subtract(first: [f32; 3], second: [f32; 3]) -> [f32; 3] {
    [
        first[0] - second[0],
        first[1] - second[1],
        first[2] - second[2],
    ]
}

#[inline]
pub fn negate(vector: [f32; 3]) -> [f32; 3] {
    [-vector[0], -vector[1], -vector[2]]
}

#[inline]
pub fn cross(first: [f32; 3], second: [f32; 3]) -> [f32; 3] {
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
pub fn dot(first: [f32; 3], second: [f32; 3]) -> f32 {
    first[0] * second[0] + first[1] * second[1] + first[2] * second[2]
}

#[inline]
pub fn scale(vector: [f32; 3], multiplier: f32) -> [f32; 3] {
    [
        vector[0] * multiplier,
        vector[1] * multiplier,
        vector[2] * multiplier,
    ]
}
