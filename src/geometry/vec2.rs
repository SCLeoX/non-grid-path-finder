use std::ops::{Add, Div, Mul};

use crate::geometry::Direction;
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
    pub fn unit(direction: Direction) -> Self {
        let angle = direction.as_angle_from_positive_x().as_radians();
        Vec2 {
            x: angle.cos(),
            y: angle.sin(),
        }
    }
    pub fn dir_mag(direction: Direction, magnitude: f64) -> Self {
        Vec2::unit(direction) * magnitude
    }
    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
    pub fn dist_squared(self, other: Vec2) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
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

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Vec2 {
    pub fn cross(self, rhs: Self) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }
    pub fn atan2(self) -> f64 {
        self.y.atan2(self.x)
    }
    pub fn direction(self) -> Direction {
        Direction::from_atan2(self.atan2())
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
