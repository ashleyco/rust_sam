#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}


impl From<Point2d<u16>> for Point2d<u32> {
    fn from(point: Point2d<u16>) -> Self {
        Point2d {
            x: point.x as u32,
            y: point.y as u32,
        }
    }
}