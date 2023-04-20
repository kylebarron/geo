use super::line_string::LineStringTrait;
use crate::{CoordNum, LineString, Polygon};
use std::iter::Cloned;
use std::slice::Iter;

pub trait PolygonTrait<'a>: Send + Sync {
    type ItemType: 'a + LineStringTrait<'a>;
    type Iter: Iterator<Item = Self::ItemType>;

    /// The exterior ring of the polygon
    fn exterior(&'a self) -> Self::ItemType;

    /// An iterator of the interior rings of this Polygon
    fn interiors(&'a self) -> Self::Iter;

    /// The number of interior rings in this Polygon
    fn num_interiors(&'a self) -> usize;

    /// Access to a specified interior ring in this Polygon
    /// Will return None if the provided index is out of bounds
    fn interior(&'a self, i: usize) -> Option<Self::ItemType>;
}

impl<'a, T: CoordNum + Send + Sync + 'a> PolygonTrait<'a> for Polygon<T> {
    type ItemType = LineString<T>;
    type Iter = Cloned<Iter<'a, Self::ItemType>>;

    fn exterior(&'a self) -> Self::ItemType {
        Polygon::exterior(self).clone()
    }

    fn interiors(&'a self) -> Self::Iter {
        Polygon::interiors(self).iter().cloned()
    }

    fn num_interiors(&'a self) -> usize {
        Polygon::interiors(self).len()
    }

    fn interior(&'a self, i: usize) -> Option<Self::ItemType> {
        Polygon::interiors(self).get(i).cloned()
    }
}

impl<'a, T: CoordNum + Send + Sync + 'a> PolygonTrait<'a> for &Polygon<T> {
    type ItemType = LineString<T>;
    type Iter = Cloned<Iter<'a, Self::ItemType>>;

    fn exterior(&'a self) -> Self::ItemType {
        Polygon::exterior(self).clone()
    }

    fn interiors(&'a self) -> Self::Iter {
        Polygon::interiors(self).iter().cloned()
    }

    fn num_interiors(&'a self) -> usize {
        Polygon::interiors(self).len()
    }

    fn interior(&'a self, i: usize) -> Option<Self::ItemType> {
        Polygon::interiors(self).get(i).cloned()
    }
}