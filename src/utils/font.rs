use ab_glyph::PxScale;
use rusttype::Scale;

pub fn get_character_dimensions(
    scale: PxScale,
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
    let scale = Scale {
        x: scale.x,
        y: scale.y,
    };

    let glyph = rtfont.glyph(character).scaled(scale);

    // Get the horizontal advance (fixed width in monospaced fonts)
    let char_width = glyph.h_metrics().advance_width.round() as u32;

    // Get the vertical height (ascender a descent), which should be the same for all characters
    let v_metrics = rtfont.v_metrics(scale);

    return (
        char_width,
        (v_metrics.ascent - v_metrics.descent).round() as u32,
    );
}
