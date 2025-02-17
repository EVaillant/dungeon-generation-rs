use crate::error::Error;
mod bsp;
pub use bsp::BspMapGenerator;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Door,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Rectangle {
    pub bottom_left: Point,
    pub bottom_right: Point,
    pub top_right: Point,
    pub top_left: Point,
}

impl Rectangle {
    pub fn from_corner(bottom_left: Point, top_right: Point) -> Self {
        Self {
            bottom_left,
            bottom_right: Point::new(top_right.x, bottom_left.y),
            top_right,
            top_left: Point::new(bottom_left.x, top_right.y),
        }
    }

    pub fn from_dimension(bottom_left: Point, width: u32, height: u32) -> Self {
        Self::from_corner(
            bottom_left,
            Point::new(bottom_left.x + width - 1, bottom_left.y + height - 1),
        )
    }

    pub fn width(&self) -> u32 {
        self.top_right.x - self.bottom_left.x + 1
    }

    pub fn height(&self) -> u32 {
        self.top_right.y - self.bottom_left.y + 1
    }
}

pub trait MapGenerator {
    fn generate(&mut self) -> Result<(), Error>;
    fn get_title(&self, x: u32, y: u32) -> TileType;
}

pub struct ChessMapGenerator {
    width: u32,
    height: u32,
}

impl ChessMapGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl MapGenerator for ChessMapGenerator {
    fn generate(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn get_title(&self, x: u32, y: u32) -> TileType {
        if x % 2 == y % 2 {
            TileType::Floor
        } else {
            TileType::Wall
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point() {
        {
            let point = Point::zero();
            assert!(point.x == 0);
            assert!(point.y == 0);
        }
        {
            let point = Point::new(5, 7);
            assert!(point.x == 5);
            assert!(point.y == 7);
        }
    }

    #[test]
    fn rectangle() {
        {
            let rectangle = Rectangle::from_corner(Point::new(5, 7), Point::new(15, 20));
            assert!(rectangle.bottom_left == Point::new(5, 7));
            assert!(rectangle.bottom_right == Point::new(15, 7));
            assert!(rectangle.top_left == Point::new(5, 20));
            assert!(rectangle.top_right == Point::new(15, 20));
            assert!(rectangle.width() == 11);
            assert!(rectangle.height() == 14);
        }
        {
            let rectangle = Rectangle::from_dimension(Point::new(5, 7), 11, 14);
            assert!(rectangle.bottom_left == Point::new(5, 7));
            assert!(rectangle.bottom_right == Point::new(15, 7));
            assert!(rectangle.top_left == Point::new(5, 20));
            assert!(rectangle.top_right == Point::new(15, 20));
            assert!(rectangle.width() == 11);
            assert!(rectangle.height() == 14);
        }
    }
}
