use ab_glyph::FontRef;
use rusttype::Scale;

use crate::character::CharacterType;

pub struct FontData<'a> {
    pub character_list: Vec<char>,
    pub font: FontRef<'a>,
    pub font_data: &'static [u8],
    pub scale: f32,
    pub character: char,
}

pub fn get_fonts<'a>(character: CharacterType) -> Result<FontData<'a>, String> {
    // if character == CharacterType::En {
    //     let font_data: &'static [u8] = include_bytes!("fonts/dejavu/DejaVuSansMono-Bold.ttf");
    //     let font = FontRef::try_from_slice(font_data).unwrap();
    //     let font_data_struct: FontData = FontData {
    //         character_list: CharacterType::En.get_character_array(),
    //         font,
    //         font_data,
    //         scale: 20.0,
    //         character:'A'
    //     };

    //     return Ok(font_data_struct);
    // } else if character == CharacterType::Simple {
    //     let font_data: &'static [u8] = include_bytes!("fonts/dejavu/DejaVuSansMono-Bold.ttf");
    //     let font = FontRef::try_from_slice(font_data).unwrap();
    //     let font_data_struct: FontData = FontData {
    //         character_list: CharacterType::Simple.get_character_array(),
    //         font,
    //         font_data,
    //         scale: 20.0,
    //         character:'.'
    //     };

    //     return Ok(font_data_struct);
    // } else if character == CharacterType::Complex {
    //     let font_data: &'static [u8] = include_bytes!("fonts/dejavu/DejaVuSansMono-Bold.ttf");
    //     let font = FontRef::try_from_slice(font_data).unwrap();
    //     let font_data_struct: FontData = FontData {
    //         character_list: CharacterType::Complex.get_character_array(),
    //         font,
    //         font_data,
    //         scale: 20.0,
    //         character:'A'
    //     };

    //     return Ok(font_data_struct);
    // } else {
    //     return Err("Unsupported language type".to_string());
    // }
    let font_data: &'static [u8] = include_bytes!("fonts/dejavu/DejaVuSansMono-Bold.ttf");
    let font = FontRef::try_from_slice(font_data).unwrap();
    let font_data_struct: FontData = FontData {
        character_list: character.get_character_array(),
        font,
        font_data,
        scale: 20.0,
        character: 'A',
    };
    return Ok(font_data_struct);
}

pub fn get_character_dimensions(
    scale: f32,
    character: char,
    font_data: &'static [u8],
) -> (u32, u32) {
    // For future me:
    // this function is somehow similiar to ImageFont.getbbox in PIL (pillow python)
    //
    // char_bbox = font.getbbox(sample_character)
    // char_width = char_bbox[2] - char_bbox[0]
    // char_height = char_bbox[3]
    //
    //
    use rusttype::Font as rFont;
    let rtfont: rFont<'static> = rFont::try_from_bytes(font_data).expect("Error constructing Font");

    let glyph = rtfont.glyph(character).scaled(Scale::uniform(scale));

    // Get the horizontal advance (fixed width in monospaced fonts)
    let char_width = glyph.h_metrics().advance_width.round() as u32;

    // Get the vertical height (ascender a descent), which should be the same for all characters
    let v_metrics = rtfont.v_metrics(Scale::uniform(scale));

    return (
        char_width,
        (v_metrics.ascent - v_metrics.descent).round() as u32,
    );
}
