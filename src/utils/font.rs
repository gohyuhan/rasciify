use core::panic;

use ab_glyph::{Font, FontRef};

use crate::character::CharacterType;

pub struct FontData<'a> {
    pub character_list: Vec<char>,
    pub font: FontRef<'a>, // Now has a lifetime parameter
    pub size: u32,
    pub scale: f32,
    pub character: char,
}

pub fn get_fonts<'a>(character: CharacterType) -> Result<FontData<'a>, String> {
    if character == CharacterType::En {
        let font_data: &'static [u8] = include_bytes!("fonts/dejavu/DejaVuSansMono-Bold.ttf");
        let font = FontRef::try_from_slice(font_data).unwrap();
        let font_data_struct: FontData = FontData {
            character_list: CharacterType::En.get_character_array(),
            font,
            size:4,
            scale: 20.0,
            character:'A'
        };

        return Ok(font_data_struct);
    } else if character == CharacterType::Simple {
        let font_data: &'static [u8] = include_bytes!("fonts/dejavu/DejaVuSansMono-Bold.ttf");
        let font = FontRef::try_from_slice(font_data).unwrap();
        let font_data_struct: FontData = FontData {
            character_list: CharacterType::Simple.get_character_array(),
            font,
            size:4,
            scale: 20.0,
            character:'.'
        };

        return Ok(font_data_struct);
    } else if character == CharacterType::Complex {
        let font_data: &'static [u8] = include_bytes!("fonts/dejavu/DejaVuSansMono-Bold.ttf");
        let font = FontRef::try_from_slice(font_data).unwrap();
        let font_data_struct: FontData = FontData {
            character_list: CharacterType::Complex.get_character_array(),
            font,
            size:4,
            scale: 20.0,
            character:'.'
        };

        return Ok(font_data_struct);
    } else {
        return Err("Unsupported language type".to_string());
    }
}

pub fn get_character_dimensions(font: &FontRef, scale: f32, character: char) -> (u32, u32) {
    let glyph_id = font.glyph_id(character);
    let scaled_glyph = font.outline_glyph(glyph_id.with_scale(scale));

    match scaled_glyph {
        Some(glyph) => {
            let bounds = glyph.px_bounds();
            let glyph_width = bounds.width();
            let glyph_height = bounds.height();
            println!("Glyph width: {}, Glyph height: {}", glyph_width, glyph_height);
            return (glyph_width as u32, glyph_height as u32);
        }
        None => {
            panic!("Warning: No outline for glyph '{}'", character);
        }
    }
}