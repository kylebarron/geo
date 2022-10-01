use crate::traits::polygon::PolygonTrait;
use arrow2::array::Array;
use arrow2::array::ListArray;
use std::slice::Iter;

use crate::arrow::line_string::{line_string_index, ArrowLineStringScalar};

/// A struct representing a non-null single Polygon geometry
#[derive(Debug, Clone)]
pub struct ArrowPolygonScalar {
    rings: ListArray<i64>,
}

impl<'a> PolygonTrait<'a> for ArrowPolygonScalar {
    type ItemType = ArrowLineStringScalar;
    type Iter = Iter<'a, Self::ItemType>;

    fn exterior(&'a self) -> Self::ItemType {
        line_string_index(&self.rings, 0).unwrap()
    }

    fn num_interiors(&'a self) -> usize {
        self.rings.len() - 1
    }

    fn interior(&'a self, i: usize) -> Option<Self::ItemType> {
        line_string_index(&self.rings, i - 1)
    }
}

pub fn polygon_index(array: &'_ ListArray<i64>, index: usize) -> Option<ArrowPolygonScalar> {
    if array.is_null(index) {
        return None;
    }

    let item = array.value(index);
    let rings = item
        .as_any()
        .downcast_ref::<ListArray<i64>>()
        .unwrap()
        .clone();
    Some(ArrowPolygonScalar { rings })
}
