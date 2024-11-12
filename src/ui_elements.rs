use crate::{shape_builder::{self, TextAttributes}, WordUnscramblerApp};
use shape_builder::{ShapeAttributes, Dimensions, RoundingType};
use eframe::{egui::{Color32, Shape, Stroke}, epaint::{Fonts, RectShape, TextShape}};
use emath::{Pos2, Rect, Vec2};
use std::{default::Default, ops::Range};
use std::ops::Mul;

// Constants for width and spcaing of letter tiles
static CONTAINER_WIDTH: f32 = 50.0;
static CONTAINER_BUFFER: f32 = CONTAINER_WIDTH + 5.0;

#[derive(Default)]

// Struct for keeping elements of UI regarding letter tiles and answer tray
pub struct UiElements{
    pub letter_squares: Vec<(Shape, char)>,
    pub answer_anchors: Vec<Pos2>,
    pub scrambled_anchors: Vec<Pos2>,
    pub trays: Vec<Shape>,
}

// Anchors established to assign letters to tiles
pub trait GenerateAnchors{
    fn scrambled_letter_anchors(&mut self) -> &mut Self;  // Scrambled letter tiles
    fn answer_letter_anchors(&mut self) -> &mut Self;     // Answer letter tiles
}

// To keep track of shapes for letter tile placement
pub trait GenerateUiShapes{
    fn place_in_scrambled(&self, position: usize) -> Shape;                           // Position for scrambled letter tile
    fn place_in_answer(&self, position: usize) -> Shape;                              // Position for answer letter tile
    fn generate_squares(&mut self, scrambled: &String, input: &String) -> &mut Self;  // Generate shapes for letter tiles
}

// Implementation for generating anchors within Word scrambler app
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

// Implementation for generating UI shapes for UI Elements struct
impl GenerateUiShapes for UiElements {

    // Function to create tile shape for scrambled letters at given anchor position
    fn place_in_scrambled(&self, position: usize) -> Shape {
        Shape::Rect(letter_square(self.scrambled_anchors[position]))
    }
    
    // FUnction to create tile shape for answer letters at given anchor position
    fn place_in_answer(&self, position: usize) -> Shape {
        Shape::Rect(letter_square(self.answer_anchors[position]))
    }

    // Function to generate the scrambled and answer letter tiles
    fn generate_squares(&mut self, scrambled: &String, input: &String) -> &mut Self {
        self.letter_squares.clear();                                  // Clears existing letter tiles
        let scrambled_chars = scrambled.chars().collect::<Vec<_>>();  // Collect scrambled letters
        let input_chars = input.chars().collect::<Vec<_>>();          // Collect user input letters

        // Generate and store tiles for each scrambled letter
        for (i, letter) in scrambled_chars.iter().enumerate() {
            self.letter_squares.push((self.place_in_scrambled(i), *letter));
        }

        // Generate and store tiles for each answer letter
        for (i, letter) in input_chars.iter().enumerate() {
            self.letter_squares.push((self.place_in_answer(i), *letter));
        }
        self
    }
}

// Function to create tile for letter at given position
pub fn letter_square(pos: Pos2) ->  RectShape{
    let attr = ShapeAttributes{
        dimensions: Dimensions::Uniform(CONTAINER_WIDTH, Pos2::from(pos)),
        fill_color: Color32::BLACK,
        rounding: RoundingType::UniformRounding(5.0),
        outline: Stroke::from((2.0, Color32::WHITE)),
    };
    RectShape::from(attr)
}

// Function to create tray shape for when answer letters are entered by user at given position
pub fn scrambled_tray(letters: usize, pos: Pos2) -> RectShape{
    let tray_width = CONTAINER_BUFFER * letters as f32;
    let attr = ShapeAttributes{
        dimensions: Dimensions::HeightWidth(
            CONTAINER_WIDTH + 10.0, tray_width + 10.0,
            pos - Vec2::from((tray_width/2.0, 0.0))),
        fill_color: Color32::DARK_GRAY,
        rounding: RoundingType::UniformRounding(3.0),
        outline: Stroke::from((2.0, Color32::LIGHT_GRAY))
    };
    RectShape::from(attr)
}

// Function to create boxes for guesses in the sidebar
pub fn guess_boxes(width: Vec2, pos: Pos2, correct: &bool) -> RectShape{
    let attr = ShapeAttributes{
        dimensions: Dimensions::HeightWidth(20.0, width.x, pos),
        fill_color: Color32::BLACK,
        rounding: RoundingType::UniformRounding(4.0),
        outline: Stroke::from((2.0, if *correct{Color32::GREEN} else {Color32::RED}))  // green outline for correct and red outline for wrong answers      
    };
    RectShape::from(attr)
}
