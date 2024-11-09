use std::time::Duration;             // Timer 
use serde::{Serialize, Deserialize}; // Used to convert to JSON for saving game
use crate::api;                      // Use dictionary API

// Structure to represent the game state with serialize and deserialize to convert to JSON to be stored for later
#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub score: u32,               // Player score
    pub time_alotted: Duration,      // Time left
    pub word_length: usize,       // Length of word to unscamble
    pub correct_answers: u8,      // Correct answer (determines word length)
    pub original_word: String,    // Original word (determines correct answer)
    pub scrambled_word: String,   // Scrambled word (determines order of letters from orig word presented to player)
    pub level: u8,                // Level (increases for every 4 words)
}

impl GameState {
    pub fn new() -> Self {
        Self {
            score: 0,                              // Score starts at 0
            time_alotted: Duration::from_secs(60),    // Start with 60 sec on clock
            word_length: 4,                        // Start by unscrambling 4 letter words
            correct_answers: 0,                    // Start at correct_answers 1 (+1 correct_answers every 4 words)
            original_word: String::new(),          // Initiate new word
            scrambled_word: String::new(),         // Scramble word
            level: 1,                              // Start at level 1 (+1 level every 4 right answers)
        }
    }

    // Async function to get a new scrambled word from dictionary (API) which updates game state
    pub fn get_new_scrambled_word(&mut self) -> &mut Self {
        if let Some((scrambled, original)) = api::get_scrambled_word(self.word_length) {
            self.original_word = original;
            self.scrambled_word = scrambled;
       }
        self
    }

    // Function to increment the word length by 1 letter every 4 correct answers
    pub fn increment_word_length(&mut self) -> &mut Self {
        if self.level % 4 == 0 {
            self.word_length += 1;
        }
        self.level += 1;
        self
    }

    // Function to handle when player inputs correct answer
    pub fn correct_answer(&mut self) -> &mut Self {
        self.score += 10;
        self.time_alotted += Duration::from_secs(5);
        self
    }

    // Function to handle when player inputs incorrect answer
    pub fn incorrect_answer(&mut self) -> &mut Self {
        if self.score >= 5 {
            self.score -= 5;
        }
        
        self.time_alotted = self.time_alotted.checked_sub(Duration::from_secs(5)).unwrap_or(Duration::ZERO);
        self
    }
}
