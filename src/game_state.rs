use std::time::Duration;
use serde::{Serialize, Deserialize};
use crate::api;

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub score: u32,
    pub time_left: Duration,
    pub word_length: usize,
    pub level: u8,
    pub original_word: String,
    pub scrambled_word: String,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            score: 0,
            time_left: Duration::from_secs(60),
            word_length: 4,
            level: 1,
            original_word: String::new(),
            scrambled_word: String::new(),
        }
    }

    pub fn get_new_scrambled_word(&mut self) {
        if let Some((original, scrambled)) = api::get_scrambled_word(self.word_length) {
            self.original_word = original;
            self.scrambled_word = scrambled;
        }
    }

    pub fn increment_word_length(&mut self) {
        if self.level % 7 == 0 {
            self.word_length += 1;
        }
        self.level += 1;
    }

    pub fn correct_answer(&mut self) {
        self.score += 10;
        self.adjust_time(1); // Add 1 second for correct answer
    }

    pub fn incorrect_answer(&mut self) {
        if self.score >= 5 {
            self.score -= 5;
        }
        self.adjust_time(-1); // Subtract 1 second for incorrect answer
    }

    fn adjust_time(&mut self, seconds: i64) {
        if seconds > 0 {
            self.time_left += Duration::from_secs(seconds as u64);
        } else {
            self.time_left = self.time_left.checked_sub(Duration::from_secs((-seconds) as u64)).unwrap_or(Duration::ZERO);
        }
    }
}
