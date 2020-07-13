use wasm_bindgen::__rt::core::slice::Iter;

use super::{IntersectSegment, Segment, Vec2};

#[derive(Debug, PartialEq)]
pub struct Shape {
    pub vertices: Vec<Vec2>,
}

impl Shape {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        Shape { vertices }
    }
    pub fn new_empty() -> Self {
        Shape { vertices: vec![] }
    }
    pub fn segments(&self) -> Segments {
        Segments { shape: self }
    }
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}

pub struct Segments<'a> {
    shape: &'a Shape,
}

impl<'a> IntoIterator for Segments<'a> {
    type Item = Segment;
    type IntoIter = SegmentsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SegmentsIter::new(self.shape)
    }
}

pub struct SegmentsIter<'a> {
    first: Vec2,
    previous: Vec2,
    iterator: Option<Iter<'a, Vec2>>,
}

impl SegmentsIter<'_> {
    fn new(shape: &Shape) -> SegmentsIter {
        let mut iterator = shape.vertices.iter();
        if let Some(&first) = iterator.next() {
            SegmentsIter {
                first,
                previous: first,
                iterator: Some(iterator),
            }
        } else {
            SegmentsIter {
                first: Vec2::zero(),
                previous: Vec2::zero(),
                iterator: None,
            }
        }
    }
}

impl Iterator for SegmentsIter<'_> {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iterator) = &mut self.iterator {
            if let Some(&next) = iterator.next() {
                let segment = Segment::new(self.previous, next);
                self.previous = next;
                Some(segment)
            } else {
                self.iterator = None;
                Some(Segment::new(self.previous, self.first))
            }
        } else {
            None
        }
    }
}

impl IntersectSegment for Shape {
    fn intersect_segment(&self, other: &Segment) -> Option<Vec2> {
        let mut closest_intersection: Option<Vec2> = None;
        let mut closest_dist_squared: f64 = f64::INFINITY;
        for segment in self.segments() {
            if let Some(intersection) = other.intersect_segment(&segment) {
                let dist = other.p0.dist_squared(intersection);
                if dist < closest_dist_squared {
                    closest_intersection = Some(intersection);
                    closest_dist_squared = dist;
                }
            }
        }
        closest_intersection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_segments_iter() {
        Iterator::eq(
            Shape {
                vertices: vec![Vec2::new(1., 2.), Vec2::new(3., 4.), Vec2::new(5., 6.)],
            }
            .segments()
            .into_iter(),
            vec![
                Segment::new_flat(1., 2., 3., 4.),
                Segment::new_flat(3., 4., 5., 6.),
                Segment::new_flat(5., 6., 1., 2.),
            ]
            .into_iter(),
        );
    }

    #[test]
    fn shape_segments_iter_one() {
        Iterator::eq(
            Shape {
                vertices: vec![Vec2::new(1., 2.)],
            }
            .segments()
            .into_iter(),
            vec![Segment::new_flat(1., 2., 1., 2.)].into_iter(),
        );
    }

    #[test]
    fn shape_segments_iter_zero() {
        Iterator::eq(
            Shape { vertices: vec![] }.segments().into_iter(),
            Vec::<Segment>::new().into_iter(),
        );
    }

    #[test]
    fn shape_intersect_segment() {
        assert_eq!(
            Segment::new_flat(5., 1., 5., 10.).intersect(&Shape::new(vec![
                Vec2::new(2., 2.),
                Vec2::new(8., 2.),
                Vec2::new(8., 8.),
                Vec2::new(2., 8.),
            ])),
            Some(Vec2::new(5., 2.))
        );
        assert_eq!(
            Segment::new_flat(5., 10., 5., 1.).intersect(&Shape::new(vec![
                Vec2::new(2., 2.),
                Vec2::new(8., 2.),
                Vec2::new(8., 8.),
                Vec2::new(2., 8.),
            ])),
            Some(Vec2::new(5., 8.))
        );
        assert_eq!(
            Segment::new_flat(1., 10., 1., 1.).intersect(&Shape::new(vec![
                Vec2::new(2., 2.),
                Vec2::new(8., 2.),
                Vec2::new(8., 8.),
                Vec2::new(2., 8.),
            ])),
            None
        );
    }
}
