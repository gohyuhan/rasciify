use std::{collections::HashMap, fs::create_dir_all, path::Path};

use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel, Rgba};
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

pub fn get_character_and_rgba_based_on_rgba(
    character_list: Vec<char>,
    img: &DynamicImage,
    cell_width: u32,
    width: u32,
    cell_height: u32,
    height: u32,
    current_row: u32,
    current_column: u32,
) -> (char, Rgba<u8>) {
    let index_scale_factor = (character_list.len() - 1) as f32 / 255.0;
    let pixel_rgba = img
        .get_pixel(
            (current_column * cell_width).min(width),
            (current_row * cell_height).min(height),
        )
        .to_rgba();

    // Get the rgb value
    let average_rgb = (pixel_rgba[0] as f32 + pixel_rgba[1] as f32 + pixel_rgba[2] as f32) / 3.0;

    // Calculate the index into the ASCII character set.
    let index = (average_rgb * index_scale_factor).round() as usize;
    return (character_list[index], pixel_rgba);
}

// this was used to sort the character based on its brightness
#[derive(Debug, Clone)]
struct CharacterBrightness {
    pub character: char,
    pub brightness: f32,
}

pub fn sort_character_brightness(
    character_list: Vec<char>,
    font_data: &'static [u8],
    scale: PxScale,
) -> Vec<char> {
    let num_char = character_list.len();
    let mut character_brightness_list: Vec<CharacterBrightness> = vec![];
    for character in character_list.clone() {
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
        let brightness =
            (img.pixels().map(|x| x[0] as f32).sum::<f32>()) / (img.pixels().len() as f32);
        character_brightness_list.push(CharacterBrightness {
            character,
            brightness,
        });
    }

    character_brightness_list.sort_by(|a, b| a.brightness.partial_cmp(&b.brightness).unwrap());

    let mut character_list_sorted: Vec<char> = vec![];
    let increment_step = (character_brightness_list[character_brightness_list.len() - 1]
        .brightness
        - character_brightness_list[0].brightness)
        / (num_char as f32);
    let mut current_value = character_brightness_list[0].brightness;
    for item in character_brightness_list.clone() {
        if item.brightness >= current_value {
            character_list_sorted.push(item.character);
            current_value += increment_step;
        }
    }

    if character_list_sorted[character_list_sorted.len() - 1]
        != character_brightness_list[character_brightness_list.len() - 1].character
    {
        character_list_sorted
            .push(character_brightness_list[character_brightness_list.len() - 1].character);
    }

    return character_list_sorted;
}

// get the flatten rgb and also the color map that we need to pass to encoder for mapping color
pub fn get_img_flatten_rgb_and_color_map(
    img_list: &Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
) -> (Vec<Vec<u8>>, Vec<u8>) {
    let mut flatten_rgb: Vec<Vec<u8>> = Vec::new();
    let mut color_map: Vec<u8> = Vec::new();
    let mut color_map_hashmap = HashMap::new();

    // Step 1: Collect RGB colors from all images
    for img in img_list {
        let mut flat_rgb_list: Vec<u8> = Vec::new();
        for pixel in img.pixels() {
            let rgb = [pixel[0], pixel[1], pixel[2]]; // Ignore alpha

            // Add to palette if it's new (up to 256 colors)
            if !color_map_hashmap.contains_key(&rgb) && color_map.len() / 3 < 256 {
                color_map_hashmap.insert(rgb, color_map.len() / 3);
                color_map.extend_from_slice(&rgb);
            }

            flat_rgb_list.extend_from_slice(&[pixel[0], pixel[1], pixel[2]]);
        }
        flatten_rgb.push(flat_rgb_list);
    }

    // Step 2: Handle palette overflow (find nearest color)
    while color_map.len() / 3 < 256 {
        color_map.extend_from_slice(&[0, 0, 0]); // Fill remaining slots with black
    }

    return (flatten_rgb, color_map);
}

// get the flatten rgb and also the color map that we need to pass to encoder for mapping color
pub fn get_img_flatten_gray_and_color_map(
    img_list: &Vec<ImageBuffer<Luma<u8>, Vec<u8>>>,
) -> (Vec<Vec<u8>>, Vec<u8>) {
    let mut flatten_gray: Vec<Vec<u8>> = Vec::new();
    let mut color_map: Vec<u8> = Vec::new();

    // Step 1: Collect gray colors from all images
    for img in img_list {
        let mut flat_gray_list: Vec<u8> = Vec::new();
        for pixel in img.pixels() {
            flat_gray_list.extend_from_slice(&[pixel[0], pixel[0], pixel[0]]);
        }
        flatten_gray.push(flat_gray_list);
    }

    // Step 2: generate a color map of grayscale
    for i in 0..=255 {
        color_map.extend_from_slice(&[i, i, i]);
    }

    return (flatten_gray, color_map);
}
