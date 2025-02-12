use std::{fs::create_dir_all, path::Path};

use ab_glyph::FontRef;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel, Rgb};
use imageproc::drawing::draw_text_mut;

use super::font::get_character_dimensions;

pub fn check_and_create_directory(output_directory: Option<&str>) -> Result<(), String> {
    if output_directory.is_some() {
        let path = Path::new(output_directory.unwrap());

        if path.exists() {
            if !path.is_dir() {
                return Err(format!(
                    "Path exists but is not a directory: {}",
                    output_directory.unwrap()
                ));
            }
        } else {
            let _ = create_dir_all(path);
        }
    }
    return Ok(());
}

pub fn get_character_line_list_based_on_luma(
    character_list: Vec<char>,
    img: &DynamicImage,
    num_cols: u32,
    cell_width: u32,
    width: u32,
    cell_height: u32,
    height: u32,
    current_row: u32,
) -> Vec<char> {
    let index_scale_factor = (character_list.len() - 1) as f32 / 255.0;
    let mut character_line_list: Vec<char> = Vec::with_capacity(num_cols as usize);

    for i in 0..num_cols {
        let pixel = img.get_pixel(
            (i * cell_width).min(width),
            (current_row * cell_height).min(height),
        );

        // Get the grayscale value (0-255)
        let luma = pixel[0];

        // Calculate the index into the ASCII character set.
        let index = (luma as f32 * index_scale_factor).round() as usize;
        character_line_list.push(character_list[index]);
    }

    return character_line_list;
}

pub fn get_character_and_rgb_based_on_rgb(
    character_list: Vec<char>,
    img: &DynamicImage,
    cell_width: u32,
    width: u32,
    cell_height: u32,
    height: u32,
    current_row: u32,
    current_column: u32,
) -> (char, Rgb<u8>) {
    let index_scale_factor = (character_list.len() - 1) as f32 / 255.0;
    let pixel_rgb = img
        .get_pixel(
            (current_column * cell_width).min(width),
            (current_row * cell_height).min(height),
        )
        .to_rgb();

    // Get the rgb value
    let average_rgb = (pixel_rgb[0] as f32 + pixel_rgb[1] as f32 + pixel_rgb[2] as f32) / 3.0;

    // Calculate the index into the ASCII character set.
    let index = (average_rgb * index_scale_factor).round() as usize;
    return (character_list[index], pixel_rgb);
}

// this was used to sort the character from less dense to more dense
// dense in here means the pixel that was taken up by a character,
// example "@" is more dense than ":"
#[derive(Debug)]
struct CharacterDensity {
    pub character: char,
    pub density: u32,
}

pub fn sort_character_density(
    character_list: Vec<char>,
    font_data: &'static [u8],
    scale: f32,
) -> Vec<char> {
    let mut character_density_list: Vec<CharacterDensity> = vec![];
    for character in character_list.clone() {
        let mut density = 0;
        let font = FontRef::try_from_slice(font_data).unwrap();

        let (width, height) = get_character_dimensions(scale, character, font_data);
        let mut img: ImageBuffer<Luma<u8>, Vec<u8>> =
            ImageBuffer::from_pixel(width, height, Luma([0]));
        draw_text_mut(
            &mut img,
            Luma([255]),
            0,
            0,
            scale,
            &font,
            &character.to_string(),
        );
        for pixel in img.pixels() {
            if pixel.0[0] != 0 {
                density += 1;
            }
        }
        character_density_list.push(CharacterDensity { character, density });
    }

    character_density_list.sort_by(|a, b| a.density.cmp(&b.density));
    let character_list_sorted = character_density_list
        .iter()
        .map(|x| x.character)
        .collect::<Vec<char>>();

    return character_list_sorted;
}
