use super::Vec2;
use crate::geometry::Sign;

pub trait Linear {
    fn p0(&self) -> Vec2;
    fn p1(&self) -> Vec2;
    fn is_horizontal(&self) -> bool {
        (self.p0().y - self.p1().y).abs() <= f64::EPSILON
    }
    fn is_vertical(&self) -> bool {
        (self.p0().x - self.p1().x).abs() <= f64::EPSILON
    }
    fn slope(&self) -> f64 {
        (self.p1().y - self.p0().y) / (self.p1().x - self.p0().x)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub p0: Vec2,
    pub p1: Vec2,
}

impl Segment {
    pub fn new(p0: Vec2, p1: Vec2) -> Segment {
        Segment { p0, p1 }
    }
    pub fn new_flat(p0x: f64, p0y: f64, p1x: f64, p1y: f64) -> Segment {
        Segment {
            p0: Vec2::new(p0x, p0y),
            p1: Vec2::new(p1x, p1y),
        }
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        (self.p0 == other.p0 && self.p1 == other.p1) || (self.p0 == other.p1 && self.p1 == other.p0)
    }
}

impl Linear for Segment {
    fn p0(&self) -> Vec2 {
        self.p0
    }
    fn p1(&self) -> Vec2 {
        self.p1
    }
}

#[inline]
fn min_max<T: PartialOrd>(v0: T, v1: T) -> (T, T) {
    if v0 < v1 {
        (v0, v1)
    } else {
        (v1, v0)
    }
}

#[inline]
fn contains<T: PartialOrd>(b0: T, b1: T, value: T) -> bool {
    let (min, max) = min_max(b0, b1);
    min <= value && value <= max
}

impl Segment {
    #[inline]
    fn find_intersection_with_segment_only_other_vertical(&self, other: &Segment) -> Option<Vec2> {
        if contains(self.p0.x, self.p1.x, other.p0.x) {
            let intersect_y = self.p0.y + (other.p0.x - self.p0.x) * self.slope();
            if contains(other.p0.y, other.p1.y, intersect_y) {
                Some(Vec2::new(other.p0.x, intersect_y))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub trait IntersectSegment {
    fn intersect_segment(&self, other: &Segment) -> Option<Vec2>;
}

impl Segment {
    pub fn intersect<T>(&self, other: &T) -> Option<Vec2>
    where
        T: IntersectSegment,
    {
        other.intersect_segment(self)
    }
}

impl IntersectSegment for Segment {
    /// Finds the intersection between self and a given segment.
    fn intersect_segment(&self, other: &Segment) -> Option<Vec2> {
        if self.is_vertical() {
            let self_x = self.p0.x;
            // Slope cannot be used
            if other.is_vertical() {
                if (self_x - other.p0.x).abs() >= f64::EPSILON {
                    return None;
                }

                // Vertical collinear
                let (min_y, max_y) = min_max(other.p0.y, other.p1.y);

                // Test for overlap
                if self.p0.y < min_y && self.p1.y < min_y {
                    return None;
                }
                if self.p0.y > max_y && self.p1.y > max_y {
                    return None;
                }

                // Use the point that is closer to self.p0
                if self.p0.y < min_y {
                    if other.p0.y < other.p1.y {
                        Some(other.p0)
                    } else {
                        Some(other.p1)
                    }
                } else if self.p0.y > max_y {
                    if other.p0.y < other.p1.y {
                        Some(other.p1)
                    } else {
                        Some(other.p0)
                    }
                } else {
                    Some(self.p0)
                }
            } else {
                // Guaranteed that `other` is not vertical
                other.find_intersection_with_segment_only_other_vertical(self)
            }
        } else if other.is_vertical() {
            self.find_intersection_with_segment_only_other_vertical(other)
        } else {
            let self_slope = self.slope();
            let other_slope = other.slope();
            if (self_slope - other_slope).abs() <= f64::EPSILON {
                let other_p0_y_interpolate_to_self_p0_x = other.p0.y - self_slope * (other.p0.x - self.p0.x);
                if (other_p0_y_interpolate_to_self_p0_x - self.p0.y).abs() >= f64::EPSILON {
                    // Parallel
                    None
                } else {
                    // Non-vertical collinear
                    let (min_x, max_x) = min_max(other.p0.x, other.p1.x);

                    // Test for overlap
                    if self.p0.x < min_x && self.p1.x < min_x {
                        return None;
                    }
                    if self.p0.x > max_x && self.p1.x > max_x {
                        return None;
                    }

                    // Use the point that is closer to self.p0
                    if self.p0.x < min_x {
                        if other.p0.x < other.p1.x {
                            Some(other.p0)
                        } else {
                            Some(other.p1)
                        }
                    } else if self.p0.x > max_x {
                        if other.p0.x < other.p1.x {
                            Some(other.p1)
                        } else {
                            Some(other.p0)
                        }
                    } else {
                        Some(self.p0)
                    }
                }
            } else {
                let self_p0_y_interpolate_to_other_p0_x = self.p0.y + self_slope * (other.p0.x - self.p0.x);
                let slope_diff = self_slope - other_slope; // How fast self catches up
                let intersect_x = other.p0.x + (other.p0.y - self_p0_y_interpolate_to_other_p0_x) / slope_diff;
                if contains(self.p0.x, self.p1.x, intersect_x) && contains(other.p0.x, other.p1.x, intersect_x) {
                    Some(Vec2::new(
                        intersect_x,
                        self.p0.y + self_slope * (intersect_x - self.p0.x),
                    ))
                } else {
                    None
                }
            }
        }
    }
}

fn overlaps(vec_self: Vec2, vec_target: Vec2) -> bool {
    if vec_self.cross(vec_target).abs() <= f64::EPSILON {
        let self_sign_x = vec_self.sign_x();
        let target_sign_x = vec_target.sign_x();
        if self_sign_x == target_sign_x {
            if self_sign_x == Sign::Zero {
                let self_sign_y = vec_self.sign_y();
                let target_sign_y = vec_target.sign_y();
                if self_sign_y == target_sign_y {
                    true
                } else {
                    target_sign_y == Sign::Zero
                }
            } else {
                true
            }
        } else {
            target_sign_x == Sign::Zero && vec_target.sign_y() == Sign::Zero
        }
    } else {
        false
    }
}

impl Segment {
    pub fn vec(&self) -> Vec2 {
        self.p1 - self.p0
    }
    pub fn vec_rev(&self) -> Vec2 {
        self.p0 - self.p1
    }
    pub fn overlaps_with_p0_to(&self, target: Vec2) -> bool {
        overlaps(self.vec(), target - self.p0)
    }
    pub fn overlaps_with_p1_to(&self, target: Vec2) -> bool {
        overlaps(self.vec_rev(), target - self.p1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_eq() {
        assert_eq!(Segment::new_flat(0., 0., 10., 10.), Segment::new_flat(0., 0., 10., 10.));
    }

    #[test]
    fn segment_ne() {
        assert_ne!(Segment::new_flat(0., 0., 10., 10.), Segment::new_flat(0., 0., 10., 11.));
    }

    #[test]
    fn segment_eq_rev() {
        assert_eq!(Segment::new_flat(0., 0., 10., 10.), Segment::new_flat(10., 10., 0., 0.));
    }

    fn test_segment_intersection(seg0: Segment, seg1: Segment, intersection: Option<Vec2>) {
        eprintln!("Intersecting {:?} with {:?}, expecting {:?}.", seg0, seg1, intersection);
        assert_eq!(seg0.intersect_segment(&seg1), intersection);
    }

    fn test_segment_intersection_both_direction(seg0: Segment, seg1: Segment, intersection: Option<Vec2>) {
        test_segment_intersection(seg0, seg1, intersection);
        test_segment_intersection(seg1, seg0, intersection);
    }

    fn test_segment_intersection_switch_vert(
        vert: (((f64, f64), (f64, f64)), ((f64, f64), (f64, f64))),
        intersection: Option<Vec2>,
    ) {
        test_segment_intersection_both_direction(
            Segment::new(((vert.0).0).into(), ((vert.0).1).into()),
            Segment::new(((vert.1).0).into(), ((vert.1).1).into()),
            intersection,
        );
        test_segment_intersection_both_direction(
            Segment::new(((vert.0).1).into(), ((vert.0).0).into()),
            Segment::new(((vert.1).0).into(), ((vert.1).1).into()),
            intersection,
        );
        test_segment_intersection_both_direction(
            Segment::new(((vert.0).0).into(), ((vert.0).1).into()),
            Segment::new(((vert.1).1).into(), ((vert.1).0).into()),
            intersection,
        );
        test_segment_intersection_both_direction(
            Segment::new(((vert.0).1).into(), ((vert.0).0).into()),
            Segment::new(((vert.1).1).into(), ((vert.1).0).into()),
            intersection,
        );
    }

    #[test]
    fn segment_find_intersection_with_segment_no_vertical() {
        test_segment_intersection_switch_vert(
            (((1., 1.), (10., 10.)), ((0., 10.), (10., 0.))),
            Some(Vec2::new(5., 5.)),
        );
        test_segment_intersection_switch_vert((((1., 1.), (5., 5.)), ((0., 10.), (10., 0.))), Some(Vec2::new(5., 5.)));
        test_segment_intersection_switch_vert((((1., 1.), (4., 4.)), ((0., 10.), (10., 0.))), None);
        test_segment_intersection_switch_vert((((1., 1.), (10., 10.)), ((0., 10.), (4., 6.))), None);
        test_segment_intersection_switch_vert((((0., 0.), (5., 5.)), ((5., 5.), (10., 0.))), Some(Vec2::new(5., 5.)));
    }

    #[test]
    fn segment_find_intersection_with_segment_one_vertical() {
        test_segment_intersection_switch_vert((((5., 5.), (5., 10.)), ((2., 8.), (8., 8.))), Some(Vec2::new(5., 8.)));
        test_segment_intersection_switch_vert(
            (((5., 5.), (5., 10.)), ((2., 10.), (8., 10.))),
            Some(Vec2::new(5., 10.)),
        );
        test_segment_intersection_switch_vert((((5., 5.), (5., 10.)), ((2., 10.), (8., 11.))), None);
    }

    #[test]
    fn segment_find_intersection_with_segment_two_vertical() {
        test_segment_intersection_switch_vert((((5., 5.), (5., 10.)), ((10., 5.), (10., 10.))), None);
    }

    #[test]
    fn segment_find_intersection_with_segment_vertical_collinear() {
        test_segment_intersection_switch_vert((((5., 5.), (5., 10.)), ((5., 15.), (5., 20.))), None);
        test_segment_intersection_switch_vert(
            (((5., 5.), (5., 10.)), ((5., 10.), (5., 15.))),
            Some(Vec2::new(5., 10.)),
        );
        test_segment_intersection(
            Segment::new_flat(5., 5., 5., 20.),
            Segment::new_flat(5., 10., 5., 15.),
            Some(Vec2::new(5., 10.)),
        );
        test_segment_intersection(
            Segment::new_flat(5., 10., 5., 20.),
            Segment::new_flat(5., 5., 5., 15.),
            Some(Vec2::new(5., 10.)),
        );
        test_segment_intersection(
            Segment::new_flat(5., 20., 5., 5.),
            Segment::new_flat(5., 10., 5., 15.),
            Some(Vec2::new(5., 15.)),
        );
        test_segment_intersection(
            Segment::new_flat(5., 15., 5., 10.),
            Segment::new_flat(5., 5., 5., 20.),
            Some(Vec2::new(5., 15.)),
        );
    }

    #[test]
    fn segment_find_intersection_with_segment_parallel() {
        test_segment_intersection_switch_vert((((5., 5.), (10., 10.)), ((10., 5.), (15., 10.))), None);
        test_segment_intersection_switch_vert((((5., 5.), (10., 5.)), ((5., 10.), (10., 10.))), None);
    }

    #[test]
    fn segment_find_intersection_with_segment_collinear() {
        test_segment_intersection_switch_vert((((5., 10.), (10., 20.)), ((15., 30.), (20., 40.))), None);
        test_segment_intersection_switch_vert(
            (((5., 10.), (10., 20.)), ((10., 20.), (15., 30.))),
            Some(Vec2::new(10., 20.)),
        );
        test_segment_intersection(
            Segment::new_flat(5., 10., 15., 30.),
            Segment::new_flat(10., 20., 20., 40.),
            Some(Vec2::new(10., 20.)),
        );
        test_segment_intersection(
            Segment::new_flat(20., 40., 10., 20.),
            Segment::new_flat(5., 10., 15., 30.),
            Some(Vec2::new(15., 30.)),
        );
        test_segment_intersection(
            Segment::new_flat(10., 20., 15., 30.),
            Segment::new_flat(5., 10., 20., 40.),
            Some(Vec2::new(10., 20.)),
        );
        test_segment_intersection(
            Segment::new_flat(15., 30., 10., 20.),
            Segment::new_flat(5., 10., 20., 40.),
            Some(Vec2::new(15., 30.)),
        );
    }

    #[test]
    fn points() {
        test_segment_intersection_switch_vert((((5., 5.), (5., 5.)), ((6., 1.), (6., 9.))), None);
        test_segment_intersection_switch_vert((((5., 5.), (5., 5.)), ((6., 1.), (7., 9.))), None);
        test_segment_intersection_switch_vert((((5., 5.), (5., 5.)), ((6., 6.), (6., 6.))), None);
        test_segment_intersection_switch_vert((((5., 5.), (5., 5.)), ((5., 5.), (5., 5.))), Some(Vec2::new(5., 5.)));
        test_segment_intersection_switch_vert((((5., 5.), (5., 5.)), ((5., 3.), (5., 8.))), Some(Vec2::new(5., 5.)));
        test_segment_intersection_switch_vert((((5., 5.), (5., 5.)), ((5., 5.), (5., 8.))), Some(Vec2::new(5., 5.)));
        test_segment_intersection_switch_vert((((5., 5.), (5., 5.)), ((5., 5.), (7., 6.))), Some(Vec2::new(5., 5.)));
    }
}
