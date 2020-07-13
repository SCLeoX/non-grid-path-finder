use std::ops::Add;

use wasm_bindgen::__rt::core::ops::Sub;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}

impl Sign {
    pub fn of(value: f64) -> Self {
        if value <= -f64::EPSILON {
            Sign::Negative
        } else if value >= f64::EPSILON {
            Sign::Positive
        } else {
            Sign::Zero
        }
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
    pub fn zero() -> Self {
        Vec2 { x: 0., y: 0. }
    }
    pub fn dist_squared(self, other: Vec2) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
    pub fn dist(self, other: Vec2) -> f64 {
        self.dist_squared(other).sqrt()
    }
    pub fn sign_x(self) -> Sign {
        Sign::of(self.x)
    }
    pub fn sign_y(self) -> Sign {
        Sign::of(self.y)
    }
    pub fn is_zero(self) -> bool {
        self.x <= f64::EPSILON && self.y <= f64::EPSILON
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vec2 {
    pub fn cross(self, rhs: Self) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Vec2 { x, y }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Vec2 {
            x: x.into(),
            y: y.into(),
        }
    }
}
