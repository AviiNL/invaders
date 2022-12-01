use crate::frame::{Drawable, Frame};

pub struct Status {
    pub score: usize,
    pub lives: usize,
    pub level: u64,
    pub paused: bool,
    pub game_over: bool,
    pub game_won: bool,
}

impl Status {
    pub fn new() -> Self {
        Self {
            score: 0,
            lives: 3,
            level: 1,
            paused: false,
            game_over: false,
            game_won: false,
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
    }

    pub fn update_lives(&mut self, lives: usize) {
        self.lives = lives;
    }

    pub fn add_score(&mut self, score: usize) {
        self.score += score;
    }
}

impl Drawable for Status {
    fn draw(&self, frame: &mut Frame) {
        frame[0][0] = 'S';
        frame[1][0] = 'c';
        frame[2][0] = 'o';
        frame[3][0] = 'r';
        frame[4][0] = 'e';
        frame[5][0] = ':';
        frame[6][0] = ' ';
        frame[7][0] = ((self.score / 100000) as u8 + 48) as char;
        frame[8][0] = ((self.score / 10000) as u8 + 48) as char;
        frame[9][0] = ((self.score / 1000) as u8 + 48) as char;
        frame[10][0] = (((self.score / 100) % 10) as u8 + 48) as char;
        frame[11][0] = (((self.score / 10) % 10) as u8 + 48) as char;
        frame[12][0] = ((self.score % 10) as u8 + 48) as char;

        frame[15][0] = 'L';
        frame[16][0] = 'i';
        frame[17][0] = 'v';
        frame[18][0] = 'e';
        frame[19][0] = 's';
        frame[20][0] = ':';
        frame[21][0] = ' ';
        frame[22][0] = ((self.lives / 10) as u8 + 48) as char;
        frame[23][0] = ((self.lives % 10) as u8 + 48) as char;

        frame[26][0] = 'L';
        frame[27][0] = 'e';
        frame[28][0] = 'v';
        frame[29][0] = 'e';
        frame[30][0] = 'l';
        frame[31][0] = ':';
        frame[32][0] = ' ';
        frame[33][0] = ((self.level / 10) as u8 + 48) as char;
        frame[34][0] = ((self.level % 10) as u8 + 48) as char;
    }
}
