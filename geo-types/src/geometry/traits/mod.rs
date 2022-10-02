pub mod line_string;
pub mod multi_line_string;
pub mod multi_point;
pub mod multi_polygon;
pub mod point;
pub mod polygon;

pub use line_string::LineStringTrait;
pub use multi_line_string::MultiLineStringTrait;
pub use multi_point::MultiPointTrait;
pub use multi_polygon::MultiPolygonTrait;
pub use point::PointTrait;
pub use polygon::PolygonTrait;
