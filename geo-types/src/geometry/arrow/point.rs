use crate::point::Point;
use arrow2::array::{Array, PrimitiveArray, StructArray};

pub fn point_index<'a>(array: &StructArray, index: usize) -> Option<Point> {
    if array.is_null(index) {
        return None;
    }

    let struct_array_values = array.values();
    let x_arrow_array = &struct_array_values[0];
    let y_arrow_array = &struct_array_values[1];

    let x_array_values = x_arrow_array
        .as_any()
        .downcast_ref::<PrimitiveArray<f64>>()
        .unwrap();
    let y_array_values = y_arrow_array
        .as_any()
        .downcast_ref::<PrimitiveArray<f64>>()
        .unwrap();

    Some(Point::new(
        x_array_values.value(index),
        y_array_values.value(index),
    ))
}
