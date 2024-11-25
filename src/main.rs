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
-11/23/2024: Adding in rodio dependency and implementing struct Sounds we have created

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


use eframe::epaint::text::cursor::Cursor;
use tokio::{self, runtime::Runtime};
use std::sync::{Arc, Mutex};
use world_scrambler::api::{WordApi, DbAPI};
use world_scrambler::contact_server::send_recieve::{self, MakeRequest, ReturnType};
use world_scrambler::game_state;
use eframe::egui::{Event, FontFamily, FontId, FontSelection};
use eframe::{App, Frame};
use eframe::egui::{self, CentralPanel, Color32, Context, text::Fonts, FontDefinitions, Key, Painter, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TopBottomPanel, Vec2};
use emath::Align2;
use world_scrambler::shape_builder::{ShapeAttributes, RoundingType, Dimensions};
use world_scrambler::ui_elements::{guess_boxes, letter_square, GenerateAnchors ,GenerateUiShapes, UiElements};
use world_scrambler::contact_server;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use world_scrambler::game_state::UpdateGameVariables;
use regex::Regex;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::SineWave;

static CONTAINER_WIDTH: f32 = 50.0;
static CONTAINER_BUFFER: f32 = CONTAINER_WIDTH + 5.0;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct WordUnscramblerApp {
    #[serde(skip)]
    pub game_state: game_state::GameState,
    pub guess_history: Vec<(String, bool)>,
    pub input_text: String,
    #[serde(skip)]
    pub timer_start: Instant,
    #[serde(skip)]
    pub validation_receiver: Option<Receiver<(String, bool)>>,
    #[serde(skip)]
    pub scrambled_word_receiver: Option<Receiver<(String, String)>>,
    pub game_over: bool,
    pub game_over_logged: bool,
    pub correct: String,
    #[serde(skip)]
    pub ui_elements: UiElements,
    #[serde(skip)]
    pub game_space: Rect,
    #[serde(skip)]
    pub user_data_base: DbAPI,
    #[serde(skip)]
    pub sounds: Sounds,
    #[serde(skip)]
    pub warning_played: bool, //we want to not have multiple warnings occur incase they improve from ten seconds left and then hit ten seconds again, warning is one time
}

pub struct Sounds {
    correct: Sink,
    incorrect: Sink,
    warning: Sink,
    game_over: Sink,
}

impl Sounds {
    pub fn new(stream: &OutputStreamHandle) -> Self {
        // Create sinks for each sound effect
        let correct = Sink::try_new(stream).unwrap();
        let incorrect = Sink::try_new(stream).unwrap();
        let warning = Sink::try_new(stream).unwrap();
        let game_over = Sink::try_new(stream).unwrap();

        // Generate tones for each sound effect
        let correct_tone = SineWave::new(880.0).take_duration(Duration::from_millis(500)); // High-pitched tone
        let incorrect_tone = SineWave::new(220.0).take_duration(Duration::from_millis(500)); // Low-pitched tone
        let warning_tone = SineWave::new(440.0).take_duration(Duration::from_millis(1000)); // Mid-pitched tone
        let game_over_tone = SineWave::new(330.0).take_duration(Duration::from_secs(2)); // Longer tone

        // Attach tones to sinks
        correct.append(correct_tone);
        incorrect.append(incorrect_tone);
        warning.append(warning_tone);
        game_over.append(game_over_tone);

        Self {
            correct,
            incorrect,
            warning,
            game_over,
        }
    }

    // Play a sound effect
    pub fn play(&self, sound_type: &str) {
        match sound_type {
            "correct" => self.correct.play(),
            "incorrect" => self.incorrect.play(),
            "warning" => self.warning.play(),
            "game_over" => self.game_over.play(),
            _ => {}
        }
    }
}


impl Default for WordUnscramblerApp {
    fn default() -> Self {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sounds = Sounds::new(&stream_handle); //had to change from _stream to stream_handle to work with OutputStreamHandle

        Self {
            //Instantiate default game values
            game_state: game_state::GameState::new(),
            guess_history: Vec::new(),
            input_text: String::new(),
            timer_start: Instant::now(),
            validation_receiver: None,
            scrambled_word_receiver: None,
            game_over: false,
            game_over_logged: false,
            correct: String::new(),
            ui_elements: UiElements::default(),
            game_space: Rect::EVERYTHING,
            user_data_base: DbAPI::new(),
            sounds,
            warning_played: false,
        }
    }
}

impl GenerateAnchors for WordUnscramblerApp {

    // Function to calculate anchors for scrambled letter tiles
    fn scrambled_letter_anchors(&mut self) -> &mut Self {
        // Clear existing anchors and recalculate based on word length
        self.ui_elements.scrambled_anchors.clear();
        let mut i: f32 = 1.0;

        // Calculate centering for scrambled letters on the screen
        for _ in 0..self.game_state.api.word_length {
            let offset = (self.game_state.api.word_length / 2) as f32 * CONTAINER_BUFFER 
                         + (self.game_state.api.word_length / 2 - 1) as f32 * 5.0 + 2.5;

            // Calculate centering for letter position within tile
            self.ui_elements.scrambled_anchors.push(
                self.game_space.center() 
                - Vec2::from((offset, 0.0)) 
                - Vec2::from((CONTAINER_BUFFER - (i * CONTAINER_BUFFER), 0.0))
            );
            i += 1.0;
        }
        self
    }

    // Function to calculate anchors for answer letter tiles
    fn answer_letter_anchors(&mut self) -> &mut Self {

        // Clear existing anchors and recalculate based on word length
        self.ui_elements.answer_anchors.clear();
        let mut i: f32 = 1.0;

        // Calculate centering for answer letters on the screen
        for _ in 0..self.game_state.api.word_length {
            let offset = (self.game_state.api.word_length / 2) as f32 * CONTAINER_BUFFER 
                         + (self.game_state.api.word_length / 2 - 1) as f32 * 5.0;

            // Calculate centering for letter position within tile
            self.ui_elements.answer_anchors.push(
                self.game_space.center_bottom() 
                - Vec2::from((offset, 0.0)) 
                - Vec2::from((CONTAINER_BUFFER - (i * CONTAINER_BUFFER), 95.0))
            );
            i += 1.0;
        }
        self
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
            
            let guesses = self.guess_history.len() as f32;
            let correct = self.guess_history.iter().filter(|&n| n.1 == true).count() as f32;
            let ratio = if guesses == 0.0 {0.0} else {correct / guesses};
            if !self.game_over_logged {
                print!("Score: {}", self.game_state.score);
                print!("Ratio: {:.2}", ratio);
                self.game_over_logged = true;
            }

            CentralPanel::default().show(ctx, |ui| {
                ui.heading("Game Over!");
                ui.label(format!("Final Score: {}", self.game_state.score));
                ui.label(format!("Correct/Incorrect Ratio: {:.2}", ratio));
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

        SidePanel::left("Friends").show(ctx, |ui|{ //Friends List
            ui.heading("Friends List");
            ui.separator();
            ui.label("Users:");

            // Create new instance of DbAPI
            let mut current_users: String = "Current Players: f".to_owned();
            let user_list = self.user_data_base.users.lock().unwrap().clone();
            for user in user_list {
                    let entry: &str = &format!(
                        "User ID: {}  \n
                        User Name: {} \n
                        Password: {} \n
                        High Score: {} \n", &user.UserID, &user.Username, &user.Password, &user.HighScore);
                current_users.push_str(entry)
            }

            ui.label(current_users);

        });//End Side Panel

        CentralPanel::default().show(ctx, |ui| { //Game Area
                self.game_space = ctx.available_rect();               
                self.scrambled_letter_anchors();
                self.answer_letter_anchors(); 

            //Static UI Elements
            ui.painter().add(world_scrambler::ui_elements::scrambled_tray(self.game_state.api.word_length, ui.ctx().available_rect().center_bottom() - Vec2::from((0.0, 100.0))));
            
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
      let input = self.input_text.trim().to_string();
      self.input_text.clear();      

      if self.game_state.original_word == input {
          self.sounds.play("correct"); //adding in sound 11/23/24
          self.game_state
              .correct_answer()
              .get_word();
          self.guess_history.push((input.clone(), true));

      } else if self.game_state.validate_word(&input) {
          self.sounds.play("correct");
          self.game_state
              .correct_answer()
              .get_word();
          self.guess_history.push((input.clone(), true));

      } else{ 
          self.sounds.play("incorrect");
          self.guess_history.push((input.clone(), false));
          self.game_state.incorrect_answer();
          self.game_state.scrambled_word = self.game_state.restore_scrambled.clone();
      }
  }
}


// Helper function to check if `input` can be formed from letters in `original`
#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default(); 
    let _ = eframe::run_native(
        "Word Unscrambler", // Set the app title
        native_options, 
        Box::new(|_cc| Ok(Box::new(WordUnscramblerApp::default()))), // Create a new WordUnscramblerApp instance
    );
}



