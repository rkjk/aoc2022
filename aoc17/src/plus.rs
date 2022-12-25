use crate::rock::{Point, Rock};

/// Row and Col here is the coordinate of the mid point
pub struct Plus {
    mid: Point
}

impl Plus {
    pub fn new(loc: Point) -> Self {
        Plus {
            mid: loc
        }
    }
}

impl Rock for Plus {
    fn get_left_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.mid;
        vec![(x - 1, y), (x, y - 1), (x, y + 1)]
    }

    fn get_right_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.mid;
        vec![(x + 1, y), (x, y + 1), (x, y - 1)]
    }

    fn get_bottom_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.mid;
        vec![(x - 1, y), (x, y - 1), (x + 1, y)]
    }

    fn get_top_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.mid;
        vec![(x - 1, y), (x, y + 1), (x + 1, y)]
    }

    fn get_highest_point(&self) -> Point {
        let (x, y) = self.mid;
        (x, y + 1)
    }

    fn get_all_points(&self) -> Vec<Point> {
        let (x, y) = self.mid;
        vec![(x, y), (x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)]
    }

    fn move_left(&mut self) {
        self.mid.0 -= 1;
    }

    fn move_right(&mut self) {
        self.mid.0 += 1;
    }

    fn move_down(&mut self) {
        self.mid.1 -= 1;
    }
}