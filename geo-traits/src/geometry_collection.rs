use super::GeometryTrait;
use geo_types::{CoordNum, Geometry, GeometryCollection};
use std::iter::Cloned;
use std::slice::Iter;

pub trait GeometryCollectionTrait {
    type T: CoordNum;
    type ItemType<'a>: 'a + GeometryTrait<T = Self::T>
    where
        Self: 'a;
    type Iter<'a>: ExactSizeIterator<Item = Self::ItemType<'a>>
    where
        Self: 'a;

    /// An iterator over the geometries in this GeometryCollection
    fn geometries(&self) -> Self::Iter<'_>;

    /// The number of geometries in this GeometryCollection
    fn num_geometries(&self) -> usize;

    /// Access to a specified geometry in this GeometryCollection
    /// Will return None if the provided index is out of bounds
    fn geometry(&self, i: usize) -> Option<Self::ItemType<'_>>;
}

impl<T: CoordNum> GeometryCollectionTrait for GeometryCollection<T> {
    type T = T;
    type ItemType<'a> = Geometry<Self::T>
    where
        Self: 'a;
    type Iter<'a> = Cloned<Iter<'a, Self::ItemType<'a>>>
    where T: 'a;

    fn geometries(&self) -> Self::Iter<'_> {
        self.0.iter().cloned()
    }

    fn num_geometries(&self) -> usize {
        self.0.len()
    }

    fn geometry(&self, i: usize) -> Option<Self::ItemType<'_>> {
        self.0.get(i).cloned()
    }
}

impl<'a, T: CoordNum> GeometryCollectionTrait for &'a GeometryCollection<T> {
    type T = T;
    type ItemType<'b> = Geometry<Self::T>     where
        Self: 'b;
    type Iter<'b> = Cloned<Iter<'a, Self::ItemType<'a>>> where
        Self: 'b;

    fn geometries(&self) -> Self::Iter<'_> {
        self.0.iter().cloned()
    }

    fn num_geometries(&self) -> usize {
        self.0.len()
    }

    fn geometry(&self, i: usize) -> Option<Self::ItemType<'_>> {
        self.0.get(i).cloned()
    }
}
