use crate::rock::{Point, Rock};

/// Row and Col here is the coordinate of the left-most point
#[derive(Debug)]
pub struct Square {
    bottom_left: Point
}

impl Square {
    pub fn new(loc: Point) -> Self {
        Square {
            bottom_left: loc
        }
    }
}

impl Rock for Square {
    fn get_left_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.bottom_left;
        vec![(x, y), (x, y + 1)]
    }

    fn get_right_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.bottom_left;
        vec![(x + 1, y), (x + 1, y + 1)]
    }

    fn get_bottom_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.bottom_left;
        vec![(x, y), (x + 1, y)]
    }

    fn get_top_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.bottom_left;
        vec![(x, y + 1), (x + 1, y + 1)]
    }

    fn get_highest_point(&self) -> Point {
        (self.bottom_left.0, self.bottom_left.1 + 1)
    }

    fn get_all_points(&self) -> Vec<Point> {
        let (x, y) = self.bottom_left;
        vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]
    }

    fn move_left(&mut self) {
        self.bottom_left.0 -= 1;
    }

    fn move_right(&mut self) {
        self.bottom_left.0 += 1;
    }

    fn move_down(&mut self) {
        self.bottom_left.1 -= 1;
    }
}