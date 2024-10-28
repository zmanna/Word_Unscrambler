/* 
Code artifacts: 
- Game Layout and User Interface
- Timer System, Word Progression Logic
- Check Word Validity
- Score/Time Calculation
- Save States

Description: Word Unscambler Game in which player has 60 seconds to unscramble randomly selected words. Each correct answer rewards 10 points
and adds a second to the clock while each wrong answer subtracts 5 points from the score and removes a second from the clock. Users must guess
the same word until the get it right. Every 7 correct words increases the word length by 1 letter.

Programmers:
- Aryamann Zutshi
- Willem Battey
- Spencer Addis
- John Mosley
- Paul Dykes

Creation Date: 10/25/2024

Dates revised:
- 10/27/2024: Build UI (Paul Dykes) and refactor code to fit UI (John Mosley, Spencer Addis, Aryamann Zutshi, Willem Battey)

Preconditions:
- User-inputted words: unscramble the presented word (String)

Postconditions:
- Correct/Incorrect: display whether user answered correctly (move to next word) or incorrectly (stay on current word) (String)

Side Effects:
- Altering the UI with new words

Invariants:
- Game loop (until timer ends)

Known Faults:
- Save feature is not integrated into the game yet

*/

mod game_state;
mod api;

use eframe::{App, Frame};
use eframe::egui::{CentralPanel, Context, Key};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct WordUnscramblerApp {
    game_state: game_state::GameState,
    input_text: String,
    #[serde(skip)]
    timer_start: Instant,
    #[serde(skip)]
    validation_receiver: Option<Receiver<(String, bool)>>,
    #[serde(skip)]
    scrambled_word_receiver: Option<Receiver<(String, String)>>,
    game_over: bool,
    correct: String,
}

impl Default for WordUnscramblerApp {
    fn default() -> Self {
        Self {
            /* 
            This implementation of the Default trait for the WordUnscramblerApp struct
            initializes the struct with the following default values:
            - game_state: A new instance of GameState from the game_state module.
            - input_text: An empty String.
            - timer_start: The current time using Instant::now().
            - validation_receiver: None, indicating no receiver is set.
            - scrambled_word_receiver: None, indicating no receiver is set.
            - game_over: false, indicating the game is not over.
            - correct: An empty String.
            */
            game_state: game_state::GameState::new(),
            input_text: String::new(),
            timer_start: Instant::now(),
            validation_receiver: None,
            scrambled_word_receiver: None,
            game_over: false,
            correct: String::new(),
        }
    }
}

impl App for WordUnscramblerApp {
    /*
    The update/3 function updates the state of the WordUnscramblerApp.
    Arguments:
    - Self: The current state of the WordUnscramblerApp.
    - Ctx: The context in which the update is occurring.
    - Frame: The frame in which the update is occurring.
    If the game is over, it displays a "Game Over!" message and the final score
    using the CentralPanel. Otherwise, it continues with the game logic.
    The function returns immediately after displaying the game over message.
     */
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.game_over {
            CentralPanel::default().show(ctx, |ui| {
                ui.heading("Game Over!");
                ui.label(format!("Final Score: {}", self.game_state.score));
            });
            return;
        }

        // Update time left
        let elapsed = self.timer_start.elapsed();
        let time_left = Duration::from_secs(60).saturating_sub(elapsed);

        if time_left <= Duration::ZERO {
            self.game_over = true;
            return;
        }

        // Handle validation receiver
        if let Some(receiver) = &self.validation_receiver {
            if let Ok((input, is_valid)) = receiver.try_recv() {
                if is_valid { // If the input is valid
                    self.game_state.correct_answer(); // Handle correct answer
                    self.game_state.increment_word_length(); // Increment word length
                    // Clear the scrambled word to fetch a new one
                    self.game_state.scrambled_word.clear(); // Clear the scrambled word
                    self.correct = "Correct!".to_string(); // Set correct message
                } else {
                    self.game_state.incorrect_answer(); // Handle incorrect answer
                    self.correct = "Incorrect".to_string(); // Set incorrect message
                    println!("{}", self.game_state.original_word); // Print the original word
                }
                self.validation_receiver = None; // Clear the validation receiver
            }
        }

        // Handle scrambled word receiver
        if let Some(receiver) = &self.scrambled_word_receiver {
            if let Ok((scrambled_word, original_word)) = receiver.try_recv() { // If scrambled word is received
                self.game_state.scrambled_word = scrambled_word; // Set the scrambled word
                self.game_state.original_word = original_word; // Set the original word
                self.scrambled_word_receiver = None; // Clear the scrambled word receiver
            }
        } else if self.game_state.scrambled_word.is_empty() { // If scrambled word is empty
            let word_length = self.game_state.word_length; // Get the word length
            let (sender, receiver) = std::sync::mpsc::channel(); // Create a new channel
            self.scrambled_word_receiver = Some(receiver); // Set the scrambled word receiver

            std::thread::spawn(move || {
                let result = match api::get_scrambled_word(word_length) { // Get a scrambled word
                    Some((scrambled_word, original_word)) => (scrambled_word, original_word), // If scrambled word is found
                    None => ("default_scrambled".to_string(), "default_original".to_string()), // If scrambled word is not found
                };
                let _ = sender.send(result); // Send the scrambled word
            });
        }

        // Build the UI
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Time left: {} seconds", time_left.as_secs())); // Display time left
            ui.heading(format!("Score: {}", self.game_state.score)); // Display score

            ui.separator(); // Add a separator

            ui.heading(format!("Scrambled Word: {}", self.game_state.scrambled_word)); // Display scrambled word

            ui.horizontal(|ui| { // Create a horizontal layout
                ui.label("Your guess: "); // Display label
                let response = ui.text_edit_singleline(&mut self.input_text); // Display text input
                response.request_focus(); // Request focus for text input
                if ui.input(|i| i.key_pressed(Key::Enter)) { // If focus is lost and Enter key is pressed
                    self.submit_input(); // Submit the input
                }
            });

            if ui.button("Submit").clicked() { // If the submit button is clicked
                self.submit_input(); // Submit the input
            }
            
            ui.heading(format!("{}", self.correct)); // Display correct/incorrect message
        });

        // Request repaint
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

impl WordUnscramblerApp {
   fn submit_input(&mut self) {
        /*  
        The submit_input/1 function processes the user's input in the WordUnscramblerApp.
        It performs the following steps:
        1. Trims and converts the input text to a string.
        2. If the input is empty, it returns immediately.
        3. Clears the input text.
        4. Clones the original word from the game state.
        5. Creates a new channel for validation.
        6. Sets the validation receiver to the newly created receiver.
        7. Spawns a background thread to validate the input:
           - Checks if the input is an exact match with the original word.
           - If not an exact match, checks if the input is a valid word and can form an anagram of the original word.
           - Sends the result (input and validation status) back through the channel.
        */
        let input = self.input_text.trim().to_string();
        if input.is_empty() {
            return;
        }
        self.input_text.clear();

        let original_word = self.game_state.original_word.clone();
        let (sender, receiver) = std::sync::mpsc::channel();
        self.validation_receiver = Some(receiver);

        // Spawn a background thread
        std::thread::spawn(move || {
            let is_exact_match = input == original_word;
            let is_valid_anagram = if !is_exact_match {
                api::is_valid_word(&input) && can_form_anagram(&input, &original_word)
            } else {
                false
            };
            let _ = sender.send((input, is_exact_match || is_valid_anagram));
        });
    }
}

// Helper function to check if `input` can be formed from letters in `original`
fn can_form_anagram(input: &str, original: &str) -> bool {
    let mut input_chars: Vec<char> = input.chars().collect(); // Convert input to a vector of characters
    let mut original_chars: Vec<char> = original.chars().collect(); // Convert original to a vector of characters
    input_chars.sort_unstable(); // Sort the input characters
    original_chars.sort_unstable(); // Sort the original characters
    input_chars == original_chars // Check if the sorted input characters match the sorted original characters
}

fn main() {
    let native_options = eframe::NativeOptions::default(); // Create default native options
    let _ = eframe::run_native( // Run the native app
        "Word Unscrambler", // Set the app title
        native_options, // Set the native options
        Box::new(|_cc| Ok(Box::new(WordUnscramblerApp::default()))), // Create a new WordUnscramblerApp instance
    );
}
