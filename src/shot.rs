use std::time::Duration;

use rusty_time::timer::Timer;

use crate::{
    frame::{Drawable, Frame, Transform, Updatable},
    NUM_ROWS,
};

pub enum Direction {
    Up,
    Down,
}

pub struct Shot {
    x: usize,
    y: usize,
    timer: Timer,
    direction: Direction,
    exploding: bool,
}

impl Shot {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self {
            x,
            y,
            timer: Timer::from_millis(75),
            direction,
            exploding: false,
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        // self.timer = Timer::from_millis(100);
    }

    pub fn dead(&self) -> bool {
        self.y == 0 || self.y == NUM_ROWS - 1 || self.exploding
    }

    fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;
        }
    }
}

impl Transform for Shot {
    fn get_x(&self) -> usize {
        self.x
    }

    fn get_y(&self) -> usize {
        self.y
    }

    fn get_width(&self) -> usize {
        1
    }

    fn get_height(&self) -> usize {
        1
    }
}

impl Updatable for Shot {
    fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready {
            match self.direction {
                Direction::Up => self.move_up(),
                Direction::Down => self.move_down(),
            }
            self.timer.reset();
        }
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = match self.direction {
            Direction::Up => '↑',
            Direction::Down => '↓',
        };
    }
}
