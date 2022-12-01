use std::io::{Stdout, Write};

use crossterm::{style::Color, terminal::*, *};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        // clear the screen
        stdout
            .queue(style::SetBackgroundColor(Color::Blue))
            .unwrap();
        stdout.queue(terminal::Clear(ClearType::All)).unwrap();

        stdout
            .queue(style::SetBackgroundColor(Color::Black))
            .unwrap();
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if last_frame[x][y] != *s || force {
                stdout.queue(cursor::MoveTo(x as u16, y as u16)).unwrap();
                stdout.queue(style::Print(*s)).unwrap();
            }
        }
    }

    stdout.flush().unwrap();
}
