use std::fmt::Debug;

/// Assume in the tunnel - left-bottom-corner is (0, 0)
/// Going right -> increase Point.0
/// Going up -> increase Point.1
//// So, the room is the first quadrant
pub type Point = (usize, usize);

pub trait Rock: Debug {
    /// Any point that can be on collision course when moving left
    /// or something is moving right towards you
    fn get_left_endpoints(&self) -> Vec<Point>;

    /// Same, when moving right, or someone else moving left
    fn get_right_endpoints(&self) -> Vec<Point>;

    /// Moving down, or someone else moving up
    fn get_bottom_endpoints(&self) -> Vec<Point>;

    /// Moving up or someone else moving down
    fn get_top_endpoints(&self) -> Vec<Point>;

    /// Get the tallest point of structure
    fn get_highest_point(&self) -> Point;

    /// Get all points of structure
    fn get_all_points(&self) -> Vec<Point>;

    fn move_left(&mut self);

    fn move_right(&mut self);

    fn move_down(&mut self);

    fn get_pivot(&self) -> Point;
}