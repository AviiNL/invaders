use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::{Drawable, Frame, Transform, Updatable};

// There are 5 different invaders, each have 2 animation frames
const INVADER_ANIMS: [[&str; 2]; 5] = [
    [" /^\\ \n|^|^|", " /^\\ \n ||| "],
    ["|_☺_ \n | ||", " _☺_|\n|| |"],
    ["~T~T~\n /~\\ ", "/P^P\\\n |~| "],
    ["/~~~\\\n\\-V-/", "/~~~\\\n /V\\ "],
    ["/___\\\n / \\ ", "\\___/\n \\V/ "],
];

pub struct Invader {
    pub is_alive: bool,
    x: usize,
    y: usize,
    anim: Vec<&'static str>,
    anim_idx: usize,
    anim_timer: Timer,
    remove: bool,
    pub score: usize,
}

impl Invader {
    pub fn new(x: usize, y: usize) -> Self {
        let anim = match y {
            2 => INVADER_ANIMS[0],
            5 => INVADER_ANIMS[1],
            8 => INVADER_ANIMS[2],
            11 => INVADER_ANIMS[3],
            14 => INVADER_ANIMS[4],
            _ => panic!("Invalid invader starting position"),
        };

        let score = match y {
            2 => 40,
            5 => 30,
            8 => 20,
            11 => 10,
            14 => 5,
            _ => panic!("Invalid invader starting position"),
        };

        Self {
            x,
            y,
            anim: anim.to_vec(),
            anim_idx: 0,
            anim_timer: Timer::from_millis(500),
            is_alive: true,
            remove: false,
            score,
        }
    }

    pub fn kill(&mut self) {
        self.is_alive = false;
        self.anim_timer = Timer::from_millis(500);
    }

    pub fn dead(&self) -> bool {
        !self.is_alive
    }

    pub fn removable(&self) -> bool {
        self.remove
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_x(&mut self, direction: isize) {
        let x = self.x as isize + direction;
        self.x = x as usize;
    }
}

impl Transform for Invader {
    fn get_x(&self) -> usize {
        self.x
    }

    fn get_y(&self) -> usize {
        self.y
    }

    fn get_width(&self) -> usize {
        5
    }

    fn get_height(&self) -> usize {
        2
    }
}

impl Updatable for Invader {
    fn update(&mut self, delta: Duration) {
        self.anim_timer.update(delta);
        if self.anim_timer.ready {
            if !self.is_alive {
                self.remove = true;
            }
            self.anim_idx = (self.anim_idx + 1) % self.anim.len();
            self.anim_timer.reset();
        }
    }
}

impl Drawable for Invader {
    fn draw(&self, frame: &mut Frame) {
        if !self.is_alive && !self.anim_timer.ready {
            // Draw the explosion
            frame[self.x + 1][self.y] = '\\';
            frame[self.x + 3][self.y] = '/';
            frame[self.x + 1][self.y + 1] = '/';
            frame[self.x + 3][self.y + 1] = '\\';

            return;
        }

        let anim = self.anim[self.anim_idx];
        let lines = anim.lines();
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                frame[self.x + j][self.y + i] = c;
            }
        }
    }
}
