use crate::rock::{Point, Rock};

/// Row and Col here is the coordinate of the left-most point of inverted L -> _|
#[derive(Debug)]
pub struct InvL {
    left: Point
}

impl InvL {
    pub fn new(loc: Point) -> Self {
        InvL {
            left: loc
        }
    }
}

impl Rock for InvL {

    fn get_left_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.left;
        vec![(x, y), (x + 2, y + 1), (x + 2, y + 2)]
    }

    fn get_right_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.left;
        vec![(x + 2, y), (x + 2, y + 1), (x + 2, y + 2)]
    }

    fn get_bottom_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.left;
        vec![(x, y), (x + 1, y), (x + 2, y)]
    }

    fn get_top_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.left;
        vec![(x, y), (x + 1, y), (x + 2, y + 2)]
    }

    fn get_highest_point(&self) -> Point {
        let (x, y) = self.left;
        (x + 2, y + 2)
    }

    fn get_all_points(&self) -> Vec<Point> {
        let (x, y) = self.left;
        vec![(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)]
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