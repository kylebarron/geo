use crate::{traits::point::PointTrait, MultiPoint, Point};
use std::slice::Iter;

pub trait MultiPointTrait<'a>: Send + Sync {
    type ItemType: 'a + PointTrait;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    // /// An iterator over the points in this MultiPoint
    // fn points(&'a self) -> Self::Iter;

    /// The number of points in this MultiPoint
    fn num_points(&'a self) -> usize;

    /// Access to a specified point in this MultiPoint
    /// Will return None if the provided index is out of bounds
    fn point(&'a self, i: usize) -> Option<Self::ItemType>;
}

impl<'a> MultiPointTrait<'a> for MultiPoint<f64> {
    type ItemType = Point;
    type Iter = Iter<'a, Self::ItemType>;

    // fn points(&'a self) -> Self::Iter {
    //     self.0.iter().map(|c| (c.x, c.y).into()).into_iter()
    // }

    fn num_points(&'a self) -> usize {
        self.0.len()
    }

    fn point(&'a self, i: usize) -> Option<Self::ItemType> {
        self.0.get(i).cloned()
    }
}
