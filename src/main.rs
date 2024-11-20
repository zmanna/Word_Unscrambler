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
-11/10/2024: Refactor code and elaborate on the UI (Spencer, John, Paul) and build SQL database (Willem) and implemnent tokio (John, Paul, Aryamann)

Preconditions:
- User-inputted words: unscramble the presented word (String)

Postconditions:
- Correct/Incorrect: display whether user answered correctly (move to next word) or incorrectly (stay on current word) (String)

Side Effects:
- Altering the UI with new words

Invariants:
- Game loop (until timer ends)
nn
Known Faults:
- Save feature is not integrated into the game yet

*/


use tokio::{self, runtime::Runtime};

use api::{ WordApi, MakeRequest};
use eframe::egui::{Event, FontFamily, FontId, FontSelection};
use eframe::{App, Frame};
use eframe::egui::{self, CentralPanel, Color32, Context, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use emath::Align2;
use shape_builder::{ShapeAttributes, RoundingType, Dimensions};
use ui_elements::{guess_boxes, letter_square, GenerateAnchors, GenerateUiShapes, UiElements};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::game_state::UpdateGameVariables;
use regex::Regex;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct WordUnscramblerApp {
    #[serde(skip)]
    game_state: game_state::GameState,
    guess_history: Vec<(String, bool)>,
    input_text: String,
    #[serde(skip)]
    timer_start: Instant,
    #[serde(skip)]
    validation_receiver: Option<Receiver<(String, bool)>>,
    #[serde(skip)]
    scrambled_word_receiver: Option<Receiver<(String, String)>>,
    game_over: bool,
    correct: String,
    #[serde(skip)]
    ui_elements: UiElements,
    #[serde(skip)]
    game_space: Rect,
}

impl Default for WordUnscramblerApp {
    fn default() -> Self {
        Self {
            //Instantiate default game values
            game_state: game_state::GameState::new(),
            guess_history: Vec::new(),
            input_text: String::new(),
            timer_start: Instant::now(),
            validation_receiver: None,
            scrambled_word_receiver: None,
            game_over: false,
            correct: String::new(),
            ui_elements: UiElements::default(),
            game_space: Rect::EVERYTHING,
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
        if self.game_state.scrambled_word.is_empty() && self.input_text.is_empty(){
            self.game_state.get_word();

            } else {
                ctx.request_repaint();
            }
        //let scrambled_word = &self.game_state.scrambled_word.clone();
        // Update time left
        let time_remaining = if let Some(res) = self.game_state.time_alotted.checked_sub(self.timer_start.elapsed()) {
            res
        }
        else{
            self.game_over = true;
            Duration::ZERO //Prevent overflow by setting duration to zero if time elapsed is greater than time alotted
        };

        if self.game_over {
            CentralPanel::default().show(ctx, |ui| {
                ui.heading("Game Over!");
                ui.label(format!("Final Score: {}", self.game_state.score));
                ui.label("Thank you for playing!");
            });
    
            // Request repaint every 100ms to keep the UI responsive
            ctx.request_repaint_after(Duration::from_millis(100));
            return; // Exit update loop since the game is over
        }

        // Build the UI
        TopBottomPanel::top("timer_bar").show(ctx, |ui|{ //Timer
            ui.heading(format!("Time left: {} seconds", time_remaining.as_secs()))
        });//End Side Panel
        
        SidePanel::right("score_and_history").show(ctx, |ui|{ //Score and Guess History
            ui.heading(format!("Score: {}", self.game_state.score));
            ui.separator();
            ui.label(format!("Guess History:"));
            let mut i = 3.0;
            for (guess, valid) in &self.guess_history{
                let guess_container = guess_boxes(ui.available_size(), Pos2::new(ctx.available_rect().right() - ui.available_size().x, 30.0 * i), valid);
                ui.painter().add(guess_container);
                i += 1.0;
                ui.painter().text(
                    guess_container.rect.center(),
                    Align2::CENTER_CENTER,
                    guess,
                    FontId::new(
                        20.0,
                        FontFamily::Monospace),
                    Color32::WHITE);
            }
        });//End Side Panel

        //SidePanel::left

        CentralPanel::default().show(ctx, |ui| { //Game Area
                self.game_space = ctx.available_rect();               
                self.scrambled_letter_anchors();
                self.answer_letter_anchors(); 

            if ui.input(|i| i.key_pressed(Key::Enter)) { // If Enter key is pressed
                        //println!("first one: {}", self.input_text);
                        //self.submit_input(); // Submit the input
                        //self.input_text.clear();
                    }

            //Static UI Elements
            ui.painter().add(ui_elements::scrambled_tray(self.game_state.api.word_length, ui.ctx().available_rect().center_bottom() - Vec2::from((0.0, 100.0))));
            
            self.ui_elements.generate_squares(&self.game_state.scrambled_word, &self.input_text);

            for (container, letter) in &self.ui_elements.letter_squares {
                match container{
                    Shape::Rect(container) => {
                        ui.painter().add(*container);
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

            ui.input(|input_state|{
                for event in &input_state.events{
                   match event{
                        Event::Text(text) => {
                            let next_char = text.chars().next().unwrap();
                            if self.game_state.scrambled_word.contains(&String::from(next_char)) {
                                self.input_text.push(next_char);
                            }
                            //eprint!("{}\n", self.input_text);
                            let re = Regex::new(&format!(r"{}", regex::escape(&next_char.to_string()))).unwrap();
                            self.game_state.scrambled_word = re.replace(&self.game_state.scrambled_word, "").to_string();},

                        Event::Key {key: egui::Key::Backspace, pressed: true, .. } => {
                            if !self.input_text.is_empty(){
                                let last_char = self.input_text.chars().next_back().unwrap();
                                self.game_state.scrambled_word.push(last_char);
                                self.input_text.remove(self.input_text.len()-1);}},

                        Event::Key {key: egui::Key::Enter, pressed: true, ..  } => {
                            self.submit_input();

                            //self.submit_input();
                            println!("Scrambled Word: {}", self.game_state.scrambled_word);
                            self.input_text.clear()},

                        _ => ()};

                }});//End Input
        });//End Central Panel

        // Request repaint
        ctx.request_repaint();
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
      self.input_text.clear();
      
      

      if self.game_state.original_word == input {
          self.game_state
              .correct_answer()
              .get_word();
          self.guess_history.push((self.input_text.clone(), true));

      } else if self.game_state.validate_word(&input) {
          self.game_state
              .correct_answer()
              .get_word();
          self.guess_history.push((self.input_text.clone(), true));

      } else{ 
          self.guess_history.push((self.input_text.clone(), false));
          self.game_state.incorrect_answer();
          self.game_state.scrambled_word = self.game_state.restore_scrambled.clone();
      }
  }
}


// Helper function to check if `input` can be formed from letters in `original`
#[tokio::main]
async fn main() {
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
                    
                });

                if ui.button("Submit").clicked() { // If the submit button is clicked
                    self.submit_input(); // Submit the input
                }
                
                ui.heading(format!("{}", self.correct)); // Display correct/incorrect message
                
            }
*/
