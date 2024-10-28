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
}

impl Default for WordUnscramblerApp {
    fn default() -> Self {
        Self {
            game_state: game_state::GameState::new(),
            input_text: String::new(),
            timer_start: Instant::now(),
            validation_receiver: None,
            scrambled_word_receiver: None,
            game_over: false,
        }
    }
}

impl App for WordUnscramblerApp {
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

        if time_left == Duration::ZERO {
            self.game_over = true;
            return;
        }

        // Handle validation receiver
        if let Some(receiver) = &self.validation_receiver {
            if let Ok((input, is_valid)) = receiver.try_recv() {
                if is_valid {
                    self.game_state.correct_answer();
                    self.game_state.increment_word_length();
                    // Clear the scrambled word to fetch a new one
                    self.game_state.scrambled_word.clear();
                } else {
                    self.game_state.incorrect_answer();
                }
                self.validation_receiver = None;
            }
        }

        // Handle scrambled word receiver
        if let Some(receiver) = &self.scrambled_word_receiver {
            if let Ok((scrambled_word, original_word)) = receiver.try_recv() {
                self.game_state.scrambled_word = scrambled_word;
                self.game_state.original_word = original_word;
                self.scrambled_word_receiver = None;
            }
        } else if self.game_state.scrambled_word.is_empty() {
            let word_length = self.game_state.word_length;
            let (sender, receiver) = std::sync::mpsc::channel();
            self.scrambled_word_receiver = Some(receiver);

            std::thread::spawn(move || {
                let result = match api::get_scrambled_word(word_length) {
                    Some((scrambled_word, original_word)) => (scrambled_word, original_word),
                    None => ("default_scrambled".to_string(), "default_original".to_string()),
                };
                let _ = sender.send(result);
            });
        }

        // Build the UI
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Time left: {} seconds", time_left.as_secs()));
            ui.heading(format!("Score: {}", self.game_state.score));

            ui.separator();

            ui.heading(format!("Scrambled Word: {}", self.game_state.scrambled_word));

            ui.horizontal(|ui| {
                ui.label("Your guess: ");
                let response = ui.text_edit_singleline(&mut self.input_text);
                if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                    self.submit_input();
                }
            });

            if ui.button("Submit").clicked() {
                self.submit_input();
            }
        });

        // Request repaint
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

impl WordUnscramblerApp {
   fn submit_input(&mut self) {
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
    let mut input_chars: Vec<char> = input.chars().collect();
    let mut original_chars: Vec<char> = original.chars().collect();
    input_chars.sort_unstable();
    original_chars.sort_unstable();
    input_chars == original_chars
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Word Unscrambler",
        native_options,
        Box::new(|_cc| Ok(Box::new(WordUnscramblerApp::default()))),
    );
}

