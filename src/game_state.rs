use std::time::Duration;             // Timer 
use serde::{Serialize, Deserialize}; // Used to convert to JSON for saving game
use std::sync::mpsc::Receiver;

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
            level: 1}                              // Start at level 1 (+1 level every 4 right answers)
    }
}

pub trait ValidateAnswer{
    fn validate_answer(&mut self, input: String);
    fn can_form_anagram(input: String, original: String) -> bool;
}
pub trait UpdateGameVariables{
    fn increment_word_length(&mut self) -> &mut Self;
    fn correct_answer(&mut self) -> &mut Self;
    fn incorrect_answer(&mut self) -> &mut Self;
    fn set_word(&mut self, scrambled: String, original: String);
    fn get_new_word(&mut self);
}

impl UpdateGameVariables for GameState{
    // Function to increment the word length by 1 letter every 4 correct answers
    fn increment_word_length(&mut self) -> &mut Self {
        if self.level % 4 == 0 {
            self.word_length += 1;
        }
        self.level += 1;
        self
    }

    // Function to handle when player inputs correct answer
    fn correct_answer(&mut self) -> &mut Self {
        self.score += 10;
        self.time_alotted += Duration::from_secs(5);
        self
    }

    // Function to handle when player inputs incorrect answer
    fn incorrect_answer(&mut self) -> &mut Self {
        if self.score >= 5 {
            self.score -= 5;
        }
        
        self.time_alotted.checked_sub(Duration::from_secs(5)).unwrap_or(Duration::ZERO);
        self
    }
    
    fn set_word(&mut self, scrambled: String, original: String){
        self.scrambled_word = scrambled;
        self.original_word = original;
    }

    fn get_new_word (&mut self){
        let (sender, receiver) = std::sync::mpsc::channel(); //Send to, and receive from the API
        let word_length = self.word_length;

        std::thread::spawn(move || { let result = 
                                     match api::get_scrambled_word(word_length) {
                                         Some((scrambled_word, original_word)) => (scrambled_word, original_word),
                                         None => ("default_scrambled".to_string(), "default_original".to_string())};
                                     let _ = sender.send(result); // result = the scrambled word
        });

        match receiver.recv(){
            Ok((scrambled, original)) => self.set_word(scrambled, original),
            Err(e) => self.set_word("ERROR".into(), "ERROR".into()),
        }
    }
}

impl ValidateAnswer for GameState{
    fn validate_answer(&mut self, input: String){
        let (sender, receiver) = std::sync::mpsc::channel();
        let original_word = self.original_word.clone();
        // Spawn a background thread
        std::thread::spawn(move || {
            let is_exact_match = input == original_word;
            let is_valid_anagram = api::is_valid_word(&input) && GameState::can_form_anagram(input.clone(), original_word);

            let _ = sender.send((input, is_exact_match || is_valid_anagram)); //Return true if exact match, or anagram
        });

        match receiver.recv() {
            Ok((_, is_valid)) =>{
                if is_valid { self.correct_answer() 
                              .increment_word_length()
                              .get_new_word();}

                else { self.incorrect_answer();
                       println!("{}", &self.original_word);}
            }

            Err(e) => eprint!("Error validating guess: {}", e)};
        
    }

    fn can_form_anagram(input: String, original: String) -> bool {
        let mut input_chars: Vec<char> = input.chars().collect(); // Convert input to a vector of characters
        let mut original_chars: Vec<char> = original.chars().collect(); // Convert original to a vector of characters
        input_chars.sort_unstable(); // Sort the input characters
        original_chars.sort_unstable(); // Sort the original characters
        input_chars == original_chars // Check if the sorted input characters match the sorted original characters
    }
}

