#[derive(Clone, Copy)]
pub enum GameStatus {
    Playing,
    Paused,
    Quit,
}

pub struct GameConfig {
    status: GameStatus,
    score: isize,
}

impl GameConfig {
    pub fn new(status: GameStatus, score: isize) -> GameConfig {
        GameConfig { status, score }
    }

    pub fn pause(&mut self) {
        self.status = GameStatus::Paused;
    }

    pub fn resume(&mut self) {
        self.status = GameStatus::Playing;
    }

    pub fn increaseScore(&mut self) {
        self.score += 1;
    }

    pub fn quit(&mut self) {
        self.status = GameStatus::Quit;
    }

    pub fn get_status(&self) -> GameStatus {
        self.status
    }
}
