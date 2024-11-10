use crate::{shape_builder::{self, TextAttributes}, WordUnscramblerApp};
use shape_builder::{ShapeAttributes, Dimensions, RoundingType};
use eframe::{egui::{Color32, Shape, Stroke}, epaint::{Fonts, RectShape, TextShape}};
use emath::{Pos2, Rect, Vec2};
use std::{default::Default, ops::Range};

static CONTAINER_WIDTH: f32 = 50.0;
static CONTAINER_BUFFER: f32 = CONTAINER_WIDTH + 5.0;

#[derive(Default)]
pub struct UiElements{
    pub letter_squares: Vec<(Shape, char)>,
    pub answer_anchors: Vec<Pos2>,
    pub scrambled_anchors: Vec<Pos2>,
    pub trays: Vec<Shape>,
}

pub trait GenerateAnchors{
    fn scrambled_letter_anchors(&mut self) -> &mut Self;
    fn answer_letter_anchors(&mut self) -> &mut Self;
}

pub trait GenerateUiShapes{
    fn place_in_scrambled(&self, position: usize) -> Shape;
    fn place_in_answer(&self, position: usize) -> Shape;
    fn generate_squares(&mut self, scrambled: &String, input: &String) -> &mut Self;
}

impl GenerateAnchors for WordUnscramblerApp{
    fn scrambled_letter_anchors(&mut self) -> &mut Self{
        let mut i: f32 = 1.0;

        for letter in self.game_state.scrambled_word.chars(){

            self.ui_elements.scrambled_anchors.push(
                self.game_space.center() - Vec2::from(((i * CONTAINER_BUFFER) - CONTAINER_BUFFER, 0.0 )));
                i += 1.0};
        self
    }

    fn answer_letter_anchors(&mut self) -> &mut Self{
        let mut i: f32 = 1.0;

        for letter in self.game_state.scrambled_word.chars(){

            self.ui_elements.answer_anchors.push(
                self.game_space.center_bottom() - Vec2::from((CONTAINER_BUFFER - (i * CONTAINER_BUFFER), 100.0 )));
            i += 1.0};
        self
    }
}

impl GenerateUiShapes for UiElements{
    fn place_in_scrambled(&self, position: usize) -> Shape {
        Shape::Rect(letter_square(self.scrambled_anchors[position]))
    }
    
    fn place_in_answer(&self, position: usize) -> Shape {
        Shape::Rect(letter_square(self.answer_anchors[position]))
    }

    fn generate_squares(&mut self, scrambled: &String, input: &String) -> &mut Self{
        self.letter_squares.clear();
        let mut scrambled_index = 0;
        let mut input_index = 0;
        for letter in scrambled.chars(){
            self.letter_squares.push(
                (self.place_in_scrambled(scrambled_index), letter));
            scrambled_index += 1;
        }
        for letter in input.chars(){
            self.letter_squares.push(
                (self.place_in_answer(input_index), letter));
            input_index += 1;
        }
        self
    }
}


pub fn letter_square(pos: Pos2) ->  RectShape{
    let attr = ShapeAttributes{
        dimensions: Dimensions::Uniform(CONTAINER_WIDTH, Pos2::from(pos)),
        fill_color: Color32::BLACK,
        rounding: RoundingType::UniformRounding(5.0),
        outline: Stroke::from((2.0, Color32::WHITE)),
    };
    RectShape::from(attr)
}

pub fn scrambled_tray(letters: usize, pos: Pos2) -> RectShape{
    let tray_width = CONTAINER_WIDTH * letters as f32;
    let attr = ShapeAttributes{
        dimensions: Dimensions::HeightWidth(
            CONTAINER_WIDTH + 10.0, tray_width + 10.0,
            pos - Vec2::from((tray_width/2.0, 0.0))),
        fill_color: Color32::LIGHT_GRAY,
        rounding: RoundingType::None,
        outline: Stroke::from((2.0, Color32::GOLD))
    };
    RectShape::from(attr)
}
