use crate::traits::point::PointTrait;
use crate::{Coordinate, LineString};

pub trait LineStringTrait: Send + Sync {
    type ItemType: PointTrait;
    // type Iter: Iterator<Item = &'a Self::ItemType>;

    // /// An iterator over the points in this LineString
    // fn points(&'a self) -> Self::Iter;

    /// The number of points in this LineString
    fn num_points(&self) -> usize;

    /// Access to a specified point in this LineString
    /// Will return None if the provided index is out of bounds
    fn point(&self, i: usize) -> Option<Self::ItemType>;
}

impl LineStringTrait for LineString<f64> {
    type ItemType = Coordinate;
    // type Iter = Iter<'a, Self::ItemType>;

    // fn points(&'a self) -> Self::Iter {
    //     self.0.iter().map(|c| (c.x, c.y).into()).into_iter()
    // }

    fn num_points(&self) -> usize {
        self.0.len()
    }

    fn point(&self, i: usize) -> Option<Self::ItemType> {
        self.0.get(i).cloned()
    }
}
