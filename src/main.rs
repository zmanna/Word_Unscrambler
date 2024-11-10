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

Creation Date: 10/25/2024F

Dates revised:
- 10/27/2024: Build UI (Paul Dykes) and refactor code to fit UI (John Mosley, Spencer Addis, Aryamann Zutshi, Willem Battey)

Preconditions:
- User-inputted words: unscramble the presented word (String)

Postconditions:
- Correct/Incorrect: display whether user answered correctly (move to next word) or incorrectly (stay on current word) (String)
00
Side Effects:
- Altering the UI with new words

Invariants:
- Game loop (until timer ends)

Known Faults:
- Save feature is not integrated into the game yet

*/

mod game_state;
mod api;
mod shape_builder;
mod ui_elements;

use eframe::egui::{FontFamily, FontId, FontSelection};
use eframe::{App, Frame};
use eframe::egui::{CentralPanel, Color32, Context, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use emath::Align2;
use shape_builder::{ShapeAttributes, RoundingType, Dimensions};
use ui_elements::letter_square;
use std::env;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::game_state::{ValidateAnswer, UpdateGameVariables};


#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct WordUnscramblerApp {
    game_state: game_state::GameState,
    input_text: String,
    guess_history: Vec<String>,
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
            //Instantiate default game values
            game_state: game_state::GameState::new(),
            input_text: String::new(),
            guess_history: Vec::new(),
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
        if self.game_state.scrambled_word.is_empty(){ self.game_state.get_new_word();}
        let scrambled_word = &self.game_state.scrambled_word;
        // Update time left
        let time_remaining = if let Some(res) = self.game_state.time_alotted.checked_sub(self.timer_start.elapsed()) {
            res
        }
        else{
            self.game_over = true;
            Duration::ZERO //Prevent overflow by setting duration to zero if time elapsed is greater than time alotted
        };

        // Build the UI
        TopBottomPanel::top("timer_bar").show(ctx, |ui|{ //Timer
            ui.heading(format!("Time left: {} seconds", time_remaining.as_secs()))
        });
        
        SidePanel::right("score_and_history").show(ctx, |ui|{ //Score and Guess History
            ui.heading(format!("Score: {}", self.game_state.score));
            ui.separator();
            ui.label(format!("Guess History: {}", self.guess_history.join("\n")))
        });

        CentralPanel::default().show(ctx, |ui| { //Game Area
            //Instantiate UI elements
            let mut i = 0.0;
            let containers_with_letter: Vec<(Shape, char)> = scrambled_word.chars().map(|letter|{ 
                i += 1.0; 
                let position = (100.0 + (i * 55.0), 100.0);

                //Create a letter container 
                (Shape::Rect(letter_square(50.0, position)), letter)
            }).collect();//Collect individual container shapes into a vector
            
            for (container, letter) in containers_with_letter {
                match container{
                    Shape::Rect(container) => {
                        ui.painter().add(container);
                        ui.painter().text(
                            container.rect.center_bottom(),//Center of container
                            Align2::CENTER_BOTTOM, 
                            letter,
                            FontId::new(
                                40.,
                                FontFamily::Monospace),
                            Color32::WHITE);},
                    _ => ()}//Return the empty container if wrong shape
            }
        
            //Display UI
            ui.painter().add(ui_elements::scrambled_tray(50.0, 1, ui.ctx().available_rect().center_bottom() - Vec2::from((0.0, 100.0))))
        });

        // Request repaint
        //ctx.request_repaint_after(Duration::from_millis(100));
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
       self.game_state.validate_answer(input);
        
    }
}

// Helper function to check if `input` can be formed from letters in `original`


fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let native_options = eframe::NativeOptions::default(); // Create default native options
    let _ = eframe::run_native( // Run the native app
        "Word Unscrambler", // Set the app title
        native_options, // Set the native options
        Box::new(|_cc| Ok(Box::new(WordUnscramblerApp::default()))), // Create a new WordUnscramblerApp instance
    );
}



/*
            if self.game_over{
                ui.heading("Game Over!");
                ui.label(format!("Final Score: {}", self.game_state.score));
                ctx.request_repaint();
                return;
            }
            else{
                ui.heading(format!("Scrambled Word: {}", scrambled_word)); // Display scrambled word

                ui.horizontal(|ui| { // Create a horizontal layout
                    ui.label("Your guess: "); // Display label
                    let response = ui.text_edit_singleline(&mut self.input_text); // Display text input
                    response.request_focus(); // Request focus for text input
                    if ui.input(|i| i.key_pressed(Key::Enter)) { // If Enter key is pressed
                        self.submit_input(); // Submit the input
                    }
                });

                if ui.button("Submit").clicked() { // If the submit button is clicked
                    self.submit_input(); // Submit the input
                }
                
                ui.heading(format!("{}", self.correct)); // Display correct/incorrect message
                
            }
*/
