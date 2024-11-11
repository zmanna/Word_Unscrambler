use std::{default, panic::Location};

use eframe::{egui::{ text::LayoutJob, Color32, FontDefinitions, FontId, Galley, Rect, Rounding, Shape, Stroke}, 
             epaint::{text::layout, FontFamily, Fonts, RectShape, TextShape}};
use emath::{Pos2, Vec2};

#[derive(Default)]
pub struct ShapeAttributes{ // Attributes for creating shapes
    pub dimensions: Dimensions, // Height/Width for shapes that have different height/width (i.e., Rectangles, Ovals)
    pub fill_color: Color32, // Fill color for the shape
    pub rounding: RoundingType, // Rounding for the shape's corners
    pub outline: Stroke, // Stroke for the shape's outline
}

pub struct TextAttributes{
    pub position: Pos2, //Position of the text
    pub text: String, //Text to be displayed
    pub size: f32, //Size of the text
    pub font: Fonts, //Font for the text
    pub underline: Stroke, //Stroke for underlines/strikethroughs etc...
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
pub enum Dimensions{ // For creating shapes with different dimensions
    HeightWidth(f32, f32, Pos2),//For shapes that have different height/width (i.e., Rectangles, Ovals)
    Uniform(f32, Pos2),//For Circles, Squares, etc...
    #[default] None,
}

impl From<Dimensions> for Rect{ //Converts Dimensions to Rect
    fn from(dimensions: Dimensions) -> Self{
        match dimensions{
            Dimensions::HeightWidth(height, width, coords) => //For shapes that have different height/width (i.e., Rectangles, Ovals)
                Self { 
                    min: coords, // Top left corner
                    max: coords.max(Pos2::from(coords + Vec2::from((width, height))))}, // Bottom right corner

            Dimensions::Uniform(size, coords) =>  // For Circles, Squares, etc...
                Self { 
                    min: coords,
                    max: coords.max(Pos2::from(coords + Vec2::from((size, size))))},

            _ => 
                Self { 
                    min: Pos2::from((0.0, 0.0)), // Default to 0,0
                    max: Pos2::from((0.0, 0.0))} // Default to 0,0
        }}
}

#[derive(Default)]
pub enum RoundingType{ // For creating shapes with different rounding
    UniformRounding(f32),
    VariableRounding(f32, f32, f32, f32),
    #[default] None,
}

impl From<RoundingType> for Rounding{ // Converts RoundingType to Rounding
    fn from(rounding_type: RoundingType) -> Self{
        match rounding_type{ // Match the rounding type
            RoundingType::UniformRounding(radius) => Self { // Assign the rounding
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






impl From<ShapeAttributes> for RectShape{ // Converts ShapeAttributes to RectShape

    fn from(attributes: ShapeAttributes) -> Self{
        Self{ rect: Rect::from(attributes.dimensions), // Convert the dimensions to a Rect
              rounding: Rounding::from(attributes.rounding), // Convert the rounding to a Rounding
              fill: attributes.fill_color, // Assign the fill color
              stroke: attributes.outline, // Assign the outline
              blur_width: 0.0, // Default blur width
              fill_texture_id: Default::default(), // Default fill texture
              uv: Rect::ZERO}} // Default UV
}

impl MorphShape for RectShape{ // Modifies RectShape after creation

    fn resize(self, height: f32, width: f32) -> Self{ // Resizes the shape
        Self { rect: self.rect.expand2(Vec2::from((height, width))),
               ..self}}

    fn move_to(self, vec_xy: Vec2) -> Self{ // Moves the shape
        Self{ rect: Rect::from_two_pos( self.rect.left_top()     + vec_xy,
                                        self.rect.right_bottom() + vec_xy),
              ..self}}
}

impl From<TextAttributes> for TextShape{ // Converts TextAttributes to TextShape

    fn from(attributes: TextAttributes) -> Self{ // Convert the attributes to a TextShape
        Self{ pos: attributes.position, // Assign the position
              galley: attributes.font.layout( // Create the text layout
                  attributes.text, // Assign the text
                  FontId::new(attributes.size, FontFamily::Proportional), // Assign the font
                  attributes.text_color, // Assign the text color
                  attributes.wrap_width), // Assign the wrap width
              underline: attributes.underline, // Assign the underline
              fallback_color: attributes.misc_color, // Assign the misc color
              override_text_color: Some(attributes.text_color), // Assign the text color
              opacity_factor: 1.0, // Default opacity
              angle: 0.0}} // Default angle
}
