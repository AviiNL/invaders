use std::time::Duration;

use rusty_time::timer::Timer;

const MAX_LIVES: usize = 3;

use crate::{
    frame::{Drawable, Frame, Transform, Updatable},
    shot::{Direction, Shot},
    NUM_COLS, NUM_ROWS,
};

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub anim: Vec<&'static str>,
    pub shots: Vec<Shot>,
    anim_idx: usize,
    anim_timer: Timer,

    pub lives: usize,
    is_alive: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: (NUM_COLS / 2) - 4,
            y: NUM_ROWS - 3,
            anim: vec![" _/^\\_ \n|#####|"],
            anim_idx: 0,
            anim_timer: Timer::from_millis(1000),
            shots: Vec::new(),
            lives: MAX_LIVES,
            is_alive: true,
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x + 6 < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 3 {
            self.shots
                .push(Shot::new(self.x + 3, self.y - 1, Direction::Up));
            true
        } else {
            false
        }
    }

    pub fn resurrect(&mut self) {
        self.is_alive = true;
    }

    pub fn die(&mut self) -> Option<usize> {
        self.is_alive = false;
        self.lives -= 1;
        if self.lives == 0 {
            None
        } else {
            Some(self.lives)
        }
    }

    pub fn dead(&self) -> bool {
        !self.is_alive
    }

    pub fn reset_lives(&mut self) {
        self.lives = MAX_LIVES;
    }
}

impl Transform for Player {
    fn get_x(&self) -> usize {
        self.x
    }

    fn get_y(&self) -> usize {
        self.y
    }

    fn get_width(&self) -> usize {
        7
    }

    fn get_height(&self) -> usize {
        2
    }
}

impl Updatable for Player {
    fn update(&mut self, delta: Duration) {
        // remove dead shots
        self.shots.retain(|shot| !shot.dead());

        for s in &mut self.shots {
            s.update(delta);
        }

        self.anim_timer.update(delta);
        if self.anim_timer.ready {
            self.anim_idx = (self.anim_idx + 1) % self.anim.len();
            self.anim_timer.reset();
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        if self.is_alive {
            let lines = self.anim[self.anim_idx].lines();
            for (y, line) in lines.enumerate() {
                for (x, c) in line.chars().enumerate() {
                    frame[self.x + x][self.y + y] = c;
                }
            }
        }

        // draw shots
        for s in &self.shots {
            s.draw(frame);
        }
    }
}
