use crate::rock::{Point, Rock};

/// Row and Col here is the coordinate of the left-most point
#[derive(Debug)]
pub struct Flat {
    left: Point
}

impl Flat {
    pub fn new(loc: Point) -> Self {
        Flat {
            left: loc
        }
    }
}

impl Rock for Flat {
    fn get_left_endpoints(&self) -> Vec<Point> {
        vec![self.left]
    }

    fn get_right_endpoints(&self) -> Vec<Point> {
        vec![(self.left.0 + 3, self.left.1)]
    }

    fn get_bottom_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.left;
        vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]
    }

    fn get_top_endpoints(&self) -> Vec<Point> {
        self.get_bottom_endpoints()
    }

    fn get_highest_point(&self) -> Point {
        self.left
    }

    fn get_all_points(&self) -> Vec<Point> {
        self.get_bottom_endpoints()
    }

    fn move_left(&mut self) {
        self.left.0 -= 1;
    }

    fn move_right(&mut self) {
        self.left.0 += 1;
    }

    fn move_down(&mut self) {
        self.left.1 -= 1;
    }
}