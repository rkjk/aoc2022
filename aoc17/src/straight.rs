use crate::rock::{Point, Rock};

/// Row and Col here is the coordinate of the bottom-most point
#[derive(Debug)]
pub struct Straight {
    bottom: Point
}

impl Straight {
    pub fn new(loc: Point) -> Self {
        Straight {
            bottom: loc
        }
    }
}

impl Rock for Straight {
    fn get_left_endpoints(&self) -> Vec<Point> {
        self.get_right_endpoints()
    }

    fn get_right_endpoints(&self) -> Vec<Point> {
        let (x, y) = self.bottom;
        vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]
    }

    fn get_bottom_endpoints(&self) -> Vec<Point> {
        vec![self.bottom]
    }

    fn get_top_endpoints(&self) -> Vec<Point> {
        vec![(self.bottom.0, self.bottom.1 + 3)]
    }

    fn get_highest_point(&self) -> Point {
        (self.bottom.0, self.bottom.1 + 3)
    }

    fn get_all_points(&self) -> Vec<Point> {
        self.get_right_endpoints()
    }
    
    fn move_left(&mut self) {
        self.bottom.0 -= 1;
    }

    fn move_right(&mut self) {
        self.bottom.0 += 1;
    }

    fn move_down(&mut self) {
        self.bottom.1 -= 1;
    }

    fn get_pivot(&self) -> Point {
        self.bottom
    }
}