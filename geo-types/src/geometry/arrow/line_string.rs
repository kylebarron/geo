use crate::traits::line_string::LineStringTrait;
use crate::Point;
use arrow2::array::Array;
use arrow2::array::{ListArray, StructArray};

use super::point::point_index;

/// A struct representing a non-null single LineString geometry
#[derive(Debug, Clone)]
pub struct ArrowLineStringScalar {
    coords: StructArray,
}

impl LineStringTrait for ArrowLineStringScalar {
    type ItemType = Point;
    // type Iter = Iter<'a, Self::ItemType>;

    fn num_points(&self) -> usize {
        self.coords.len()
    }

    fn point(&self, index: usize) -> Option<Self::ItemType> {
        point_index(&self.coords, index)
    }
}

pub fn line_string_index(array: &'_ ListArray<i64>, index: usize) -> Option<ArrowLineStringScalar> {
    if array.is_null(index) {
        return None;
    }

    let item = array.value(index);
    let coords = item.as_any().downcast_ref::<StructArray>().unwrap().clone();
    Some(ArrowLineStringScalar { coords })
}
