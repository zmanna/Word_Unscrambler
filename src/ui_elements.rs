use crate::shape_builder::{self, TextAttributes};
use shape_builder::{ShapeAttributes, Dimensions, RoundingType};
use eframe::{egui::{Color32, Stroke}, epaint::{Fonts, RectShape, TextShape}};
use emath::{Vec2, Pos2};

pub fn letter_square(size: f32, pos: (f32, f32)) ->  RectShape{
    let attr = ShapeAttributes{
        dimensions: Dimensions::Uniform(size, Pos2::from(pos)),
        fill_color: Color32::BLACK,
        rounding: RoundingType::UniformRounding(5.0),
        outline: Stroke::from((2.0, Color32::WHITE)),
    };
    RectShape::from(attr)
}

pub fn scrambled_tray(letter_width: f32, letters: u8, pos: Pos2) -> RectShape{
    let tray_width = letter_width * f32::from(letters);
    let attr = ShapeAttributes{
        dimensions: Dimensions::HeightWidth(letter_width + 10.0, tray_width + 10.0, pos),
        fill_color: Color32::LIGHT_GRAY,
        rounding: RoundingType::None,
        outline: Stroke::from((2.0, Color32::GOLD))
    };
    RectShape::from(attr)
}


/*
pub fn letter_content(owner_position: (f32, f32), contents: String, font_library: Fonts) -> TextShape{
    let attr = TextAttributes{
        position: Pos2::from((owner_position.0 + 10.0, owner_position.1 + 10.0)),
        text: contents,
        size: 20.0,
        font: font_library,
        underline: Stroke::default(), //None by default
        misc_color: Color32::TRANSPARENT,
        text_color: Color32::WHITE,
        wrap_width: 1.0,
    };
    TextShape::from(attr)
}
*/
