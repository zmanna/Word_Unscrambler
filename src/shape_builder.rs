use std::{default, panic::Location};

use eframe::{egui::{ text::LayoutJob, Color32, FontDefinitions, FontId, Galley, Rect, Rounding, Shape, Stroke}, 
             epaint::{text::layout, FontFamily, Fonts, RectShape, TextShape}};
use emath::{Pos2, Vec2};

#[derive(Default)]
pub struct ShapeAttributes{
    pub dimensions: Dimensions,
    pub fill_color: Color32,
    pub rounding: RoundingType,
    pub outline: Stroke,
}

pub struct TextAttributes{
    pub position: Pos2,
    pub text: String,
    pub size: f32,
    pub font: Fonts,
    pub underline: Stroke,
    pub misc_color: Color32, //Color for underlines/strikethroughs etc... (Also sets the text color if none is given)
    pub text_color: Color32, //Does not set the color of non-text elements
    pub wrap_width: f32, //Wrap text after this number of characters
}

impl Default for TextAttributes{
    fn default() -> Self{
        Self{
            position: Default::default(),
            text: Default::default(),
            size: Default::default(),
            font: Fonts::new(10.0, 10, FontDefinitions::default()),
            underline: Default::default(),
            misc_color: Default::default(),
            text_color: Default::default(),
            wrap_width: Default::default()}}
}

#[derive(Default)]
pub enum Dimensions{
    HeightWidth(f32, f32, Pos2),//For shapes that have different height/width (i.e., Rectangles, Ovals)
    Uniform(f32, Pos2),//For Circles, Squares, etc...
    #[default] None,
}

impl From<Dimensions> for Rect{
    fn from(dimensions: Dimensions) -> Self{
        match dimensions{
            Dimensions::HeightWidth(height, width, coords) => 
                Self { 
                    min: coords,
                    max: coords.max(Pos2::from(coords + Vec2::from((height, width))))},

            Dimensions::Uniform(size, coords) => 
                Self { 
                    min: coords,
                    max: coords.max(Pos2::from(coords + Vec2::from((size, size))))},

            _ => 
                Self { 
                    min: Pos2::from((0.0, 0.0)),
                    max: Pos2::from((0.0, 0.0))}
        }}
}

#[derive(Default)]
pub enum RoundingType{
    UniformRounding(f32),
    VariableRounding(f32, f32, f32, f32),
    #[default] None,
}

impl From<RoundingType> for Rounding{
    fn from(rounding_type: RoundingType) -> Self{
        match rounding_type{
            RoundingType::UniformRounding(radius) => Self {
                nw: radius,
                ne: radius,
                sw: radius,
                se: radius}, //Uniform rounding for all corners

            RoundingType::VariableRounding(nw_rad, ne_rad, sw_rad, se_rad) => Self {
                nw: nw_rad,
                ne: ne_rad,
                sw: sw_rad,
                se: se_rad}, //Assign all corner roundings individually

            _ => Self {
                nw: 0.0,
                ne: 0.0,
                sw: 0.0,
                se: 0.0} //No rounding
        }}
}



trait MorphShape{ //For modifying shapes after creation
    fn resize(self, height: f32, width: f32) -> Self;
    fn move_to(self, vec_xy: Vec2) -> Self;
}






impl From<ShapeAttributes> for RectShape{

    fn from(attributes: ShapeAttributes) -> Self{
        Self{ rect: Rect::from(attributes.dimensions),
              rounding: Rounding::from(attributes.rounding),
              fill: attributes.fill_color,
              stroke: attributes.outline,
              blur_width: 0.0,
              fill_texture_id: Default::default(),
              uv: Rect::ZERO}}
}

impl MorphShape for RectShape{

    fn resize(self, height: f32, width: f32) -> Self{
        Self { rect: self.rect.expand2(Vec2::from((height, width))),
               ..self}}

    fn move_to(self, vec_xy: Vec2) -> Self{
        Self{ rect: Rect::from_two_pos( self.rect.left_top()     + vec_xy,
                                        self.rect.right_bottom() + vec_xy),
              ..self}}
}

impl From<TextAttributes> for TextShape{

    fn from(attributes: TextAttributes) -> Self{
        Self{ pos: attributes.position,
              galley: attributes.font.layout(
                  attributes.text,
                  FontId::new(attributes.size, FontFamily::Proportional),
                  attributes.text_color,
                  attributes.wrap_width),
              underline: attributes.underline,
              fallback_color: attributes.misc_color,
              override_text_color: Some(attributes.text_color),
              opacity_factor: 1.0,
              angle: 0.0}}
}
