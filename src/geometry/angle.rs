use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Sub};

fn bound_direction(direction: f64) -> f64 {
    if direction > PI {
        direction - 2. * PI
    } else if direction <= -PI {
        direction + 2. * PI
    } else {
        direction
    }
}

fn bound_angle(angle: f64) -> f64 {
    if angle > 2. * PI {
        angle - 2. * PI
    } else if angle <= 0. {
        angle + 2. * PI
    } else {
        angle
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
// (-PI, PI]
pub struct Direction(f64);

impl Direction {
    pub fn from_atan2(atan2: f64) -> Self {
        debug_assert!(atan2 > -PI && atan2 <= PI);
        Direction(atan2)
    }
    pub fn as_angle_from_positive_x(self) -> Angle {
        Angle(bound_angle(self.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
// (0, 2PI]
pub struct Angle(f64);

impl Angle {
    pub fn as_radians(self) -> f64 {
        self.0
    }
    pub fn from_radians_bounded(radians: f64) -> Angle {
        debug_assert!(radians > 0. && radians <= 2. * PI);
        Angle(radians)
    }
    pub fn explementary(self) -> Angle {
        Angle(2. * PI - self.0)
    }
}

impl Sub<Direction> for Direction {
    type Output = Angle;

    fn sub(self, rhs: Direction) -> Self::Output {
        Angle(bound_angle(self.0 - rhs.0))
    }
}

impl Sub<Angle> for Direction {
    type Output = Direction;

    fn sub(self, rhs: Angle) -> Self::Output {
        Direction(bound_direction(self.0 - rhs.0))
    }
}

impl Add<Angle> for Direction {
    type Output = Direction;

    fn add(self, rhs: Angle) -> Self::Output {
        Direction(bound_direction(self.0 + rhs.0))
    }
}

impl AddAssign<Angle> for Direction {
    fn add_assign(&mut self, rhs: Angle) {
        *self = *self + rhs;
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {
        Angle(bound_angle(self.0 + rhs.0))
    }
}
