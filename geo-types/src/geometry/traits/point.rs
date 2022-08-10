use crate::{Coordinate, Point};

pub trait PointTrait: Send + Sync {
    /// x component of this point
    fn x(&self) -> f64;

    /// y component of this point
    fn y(&self) -> f64;
}

impl PointTrait for Point<f64> {
    fn x(&self) -> f64 {
        self.0.x
    }
    fn y(&self) -> f64 {
        self.0.y
    }
}

impl PointTrait for Coordinate<f64> {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}
