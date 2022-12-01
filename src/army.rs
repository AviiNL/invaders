use std::{cmp::max, time::Duration};

use rand::Rng;
use rusty_time::timer::Timer;

use crate::{
    frame::{Drawable, Frame, Transform, Updatable},
    invader::Invader,
    shot::Shot,
    NUM_COLS, NUM_ROWS,
};

pub struct Army {
    pub invaders: Vec<Invader>,
    pub shots: Vec<Shot>,
    direction: isize,
    move_timer: Timer,
    shot_timer: Timer,
}

impl Army {
    pub fn new(level: u64) -> Self {
        let mut invaders = Vec::new();
        for i in 0..9 {
            for j in 0..5 {
                invaders.push(Invader::new(9 + i * 7, 2 + j * 3));
            }
        }

        let move_timer_time = max(250, 1100 - level * 100);

        Self {
            invaders,
            shots: Vec::new(),
            direction: 1,
            move_timer: Timer::from_millis(move_timer_time),
            shot_timer: Timer::from_millis(4000),
        }
    }

    pub fn move_army(&mut self) {
        if self.invaders.is_empty() {
            return;
        }

        let mut downwards = false;

        if self.direction == -1 {
            let min_x = self.invaders.iter().map(|i| i.get_x()).min().unwrap();
            if min_x == 0 {
                self.direction = 1;
                downwards = true;
            }
        } else {
            let max_x = self
                .invaders
                .iter()
                .map(|i| i.get_x() + i.get_width())
                .max()
                .unwrap();
            if max_x == NUM_COLS - 1 {
                self.direction = -1;
                downwards = true;
            }
        }

        if downwards {
            let new_duration = max(250, self.move_timer.duration.as_millis() - 100);
            self.move_timer = Timer::from_millis(new_duration as u64);
        }

        for invader in &mut self.invaders {
            if downwards {
                invader.move_down();
            } else {
                invader.move_x(self.direction);
            }
        }
    }

    pub fn check_collision(&mut self, other: &dyn Transform) -> Option<usize> {
        for invader in &mut self.invaders {
            if invader.is_alive && invader.check_collision(other) {
                invader.kill();
                return Some(invader.score);
            }
        }
        None
    }

    pub fn get_shooter(&self) -> Option<&Invader> {
        if self.invaders.is_empty() {
            return None;
        }

        let index = rand::thread_rng().gen_range(0..self.invaders.len());

        let invader = &self.invaders[index];

        if invader.is_alive {
            return Some(invader);
        }

        None
    }

    pub fn all_dead(&self) -> bool {
        self.invaders.is_empty()
    }

    pub fn invaded(&self) -> bool {
        self.invaders
            .iter()
            .any(|i| i.get_y() + i.get_height() >= NUM_ROWS - 1)
    }
}

impl Updatable for Army {
    fn update(&mut self, delta: Duration) {
        self.move_timer.update(delta);
        self.shot_timer.update(delta);

        self.invaders.retain(|invader| !invader.removable());
        for invader in &mut self.invaders {
            invader.update(delta);
        }

        if self.move_timer.ready {
            self.move_army();
            self.move_timer.reset();
        }

        if self.shot_timer.ready {
            if let Some(invader) = self.get_shooter() {
                self.shots.push(Shot::new(
                    invader.get_x() + 2,
                    invader.get_y() + 2,
                    crate::shot::Direction::Down,
                ));
            }
            // reset timer with a random value between 500 and 8000
            self.shot_timer = Timer::from_millis(rand::thread_rng().gen_range(500..8000));
            self.shot_timer.reset();
        }

        self.shots.retain(|shot| !shot.dead());
        for shot in &mut self.shots {
            shot.update(delta);
        }
    }
}

impl Drawable for Army {
    fn draw(&self, frame: &mut Frame) {
        for invader in &self.invaders {
            invader.draw(frame);
        }

        for shot in &self.shots {
            shot.draw(frame);
        }
    }
}
