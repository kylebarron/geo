use crate::CoordNum;
use crate::{Coord, LineString};

use super::point::PointTrait;
use std::iter::Cloned;
use std::slice::Iter;

pub trait LineStringTrait<'a>: Send + Sync {
    type T: CoordNum + Send + Sync;
    type ItemType: 'a + PointTrait<T = Self::T> + PartialEq;
    type Iter: ExactSizeIterator + Iterator<Item = Self::ItemType>;

    type LineItemType
    type LineIter

    /// An iterator over the points in this LineString
    fn points(&'a self) -> Self::Iter;

    /// The number of points in this LineString
    fn num_points(&'a self) -> usize;

    /// Access to a specified point in this LineString
    /// Will return None if the provided index is out of bounds
    fn point(&'a self, i: usize) -> Option<Self::ItemType>;

    fn first_point(&'a self) -> Option<Self::ItemType> {
        if self.num_points() == 0 {
            None
        } else {
            self.point(0)
        }
    }

    fn last_point(&'a self) -> Option<Self::ItemType> {
        if self.num_points() == 0 {
            None
        } else {
            self.point(self.num_points() - 1)
        }
    }

    fn lines(&'a self) -> impl ExactSizeIterator + Iterator<Item = Line<T>> + '_;
}

impl<'a, T: CoordNum + Send + Sync + 'a> LineStringTrait<'a> for LineString<T> {
    type T = T;
    type ItemType = Coord<T>;
    type Iter = Cloned<Iter<'a, Self::ItemType>>;

    fn points(&'a self) -> Self::Iter {
        self.0.iter().cloned()
    }

    fn num_points(&self) -> usize {
        self.0.len()
    }

    fn point(&'a self, i: usize) -> Option<Self::ItemType> {
        self.0.get(i).cloned()
    }
}

impl<'a, T: CoordNum + Send + Sync + 'a> LineStringTrait<'a> for &LineString<T> {
    type T = T;
    type ItemType = Coord<T>;
    type Iter = Cloned<Iter<'a, Self::ItemType>>;

    fn points(&'a self) -> Self::Iter {
        self.0.iter().cloned()
    }

    fn num_points(&self) -> usize {
        self.0.len()
    }

    fn point(&'a self, i: usize) -> Option<Self::ItemType> {
        self.0.get(i).cloned()
    }
}
