use std::time::Duration;
use api::WordApi;
use crate::contact_server::send_recieve::{MakeRequest, ReturnType};

// Structure to represent the game state with serialize and deserialize to convert to JSON to be stored for later
pub struct GameState {
    pub score: u32,               // Player score
    pub time_alotted: Duration,      // Time left
    pub original_word: String,    // Original word (determines correct answer)
    pub scrambled_word: String,   // Scrambled word (determines order of letters from orig word presented to player)
    pub restore_scrambled: String,
    pub requested: bool,
    pub api: WordApi,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            score: 0,                              // Score starts at 0
            time_alotted: Duration::from_secs(60),    // Start with 60 sec on clock
            original_word: String::new(),          // Initiate new word
            scrambled_word: String::new(),         // Scramble word
            restore_scrambled: String::new(),       // scrambled word for restoring when user gets it wrong
            requested: false,
            api: WordApi::default(),
        }
    }
    fn validate_word(&self, input: &str) -> bool {
        match self.api.send_request(input){
            ReturnType::IsValid(valid) => valid,
            _ =>{eprint!("Error validating word..."); false}
        }
    }
}

pub trait UpdateGameVariables{
    fn correct_answer(&mut self) -> &mut Self;
    fn incorrect_answer(&mut self) -> &mut Self;
    fn get_word(&mut self);
}

impl UpdateGameVariables for GameState{
    // Function to handle when player inputs correct answer
    fn correct_answer(&mut self) -> &mut Self {
        self.score += 10;
        self.time_alotted += Duration::from_secs(5);
        self
    }

    // Function to handle when player inputs incorrect answer
    fn incorrect_answer(&mut self) -> &mut Self {
        if self.score >= 5 { self.score -= 5 };
        self.time_alotted.checked_sub(Duration::from_secs(5)).unwrap_or(Duration::ZERO);
        self
    }
    
    fn get_word(&mut self){
        if let Some((scrambled_word, original_word)) = self.api.get_next_word(){
            self.restore_scrambled = scrambled_word.clone();
            self.scrambled_word = scrambled_word;
            self.original_word = original_word;
            println!("{}, {}", self.scrambled_word, self.original_word);
            self.api.requested = false;
        }else{
            self.api.requested = true;
        }
    }
}
