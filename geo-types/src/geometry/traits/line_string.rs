use crate::traits::point::PointTrait;
use crate::{LineString, Point};
use std::slice::Iter;

pub trait LineStringTrait<'a>: Send + Sync {
    type ItemType: 'a + PointTrait;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    // /// An iterator over the points in this LineString
    // fn points(&'a self) -> Self::Iter;

    /// The number of points in this LineString
    fn num_points(&'a self) -> usize;

    /// Access to a specified point in this LineString
    /// Will return None if the provided index is out of bounds
    fn point(&'a self, i: usize) -> Option<Self::ItemType>;
}

impl<'a> LineStringTrait<'a> for LineString<f64> {
    type ItemType = Point;
    type Iter = Iter<'a, Self::ItemType>;

    // fn points(&'a self) -> Self::Iter {
    //     self.0.iter().map(|c| (c.x, c.y).into()).into_iter()
    // }

    fn num_points(&'a self) -> usize {
        self.0.len()
    }

    fn point(&'a self, i: usize) -> Option<Self::ItemType> {
        let p: Point = (self[i].x, self[i].y).into();
        Some(p)
    }
}
