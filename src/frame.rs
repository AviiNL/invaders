use std::time::Duration;

use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<char>>;

pub fn new_frame() -> Frame {
    vec![vec![' '; NUM_ROWS]; NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

pub trait Updatable {
    fn update(&mut self, delta: Duration);
}

pub trait Transform {
    fn get_x(&self) -> usize;
    fn get_y(&self) -> usize;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn check_collision(&self, other: &dyn Transform) -> bool {
        let x = self.get_x();
        let y = self.get_y();
        let width = self.get_width();
        let height = self.get_height();
        let other_x = other.get_x();
        let other_y = other.get_y();
        let other_width = other.get_width();
        let other_height = other.get_height();
        x < other_x + other_width
            && x + width > other_x
            && y < other_y + other_height
            && y + height > other_y
    }
}
