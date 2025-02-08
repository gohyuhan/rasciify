use std::{
    fs::{create_dir_all, write},
    path::Path,
};

use ab_glyph::PxScale;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};
use imageproc::drawing::draw_text_mut;
use ndarray::ArrayView1;

use crate::{
    character::CharacterType,
    utils::{
        font::{get_character_dimensions, get_fonts, FontData},
        utils::{convert_grayscale_img_to_ndarray, get_character_line_list_based_on_luma},
    },
};

/// Converts an image to ASCII art.
pub fn image_to_image(
    path: &str,
    num_cols: u32,
    character_type: CharacterType,
    output_directory: Option<&str>,
    filename: Option<&str>,
) -> Result<String, String> {
    let img = image::open(path).expect("Failed to open image");
    let gray_img = img.grayscale();

    let ascii_img = grayscale_to_ascii_img(&gray_img, num_cols, character_type);

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

    if filename.is_some() {
        let mut path: String = format!("{}.jpg", filename.unwrap());
        if output_directory.is_some() {
            path = format!("{}{}", output_directory.unwrap(), filename.unwrap());
        }
        ascii_img.save(path).unwrap();
    }

    return Ok("Done".to_string());
}

fn grayscale_to_ascii_img(
    img: &DynamicImage,
    num_cols: u32,
    character_type: CharacterType,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let character_data: FontData = get_fonts(character_type).unwrap();
    let mut num_cols = num_cols;
    // width per cell
    let mut cell_width = width / num_cols;
    // height per cell
    let mut cell_height = 2 * cell_width;
    let mut num_rows = height / cell_height;

    if num_cols > width || num_rows > height {
        // Too many columns or rows. Use default setting
        cell_width = 6;
        cell_height = 12;
        num_cols = width / cell_width;
        num_rows = height / cell_height;
    }

    // Calculate the size of the output image based on the number of columns and rows need and the size of the font
    let (char_width, char_height) = get_character_dimensions(
        character_data.scale,
        character_data.character,
        character_data.font_data,
    );
    let output_image_width = char_width * num_cols;
    let output_image_height = char_height * num_rows;

    // create a blank image to draw the ASCII art on
    let mut out_image: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::new(output_image_width, output_image_height);

    // Fill the background with the specified color
    for pixel in out_image.pixels_mut() {
        *pixel = Luma([0]);
    }

    // convert gray scale image to array
    let img_array = convert_grayscale_img_to_ndarray(&img);

    // Get the character based on the grayscale value and form a row of characters and draw it on the image row by row
    for i in 0..num_rows {
        let img_row: ArrayView1<u8> = img_array.row((i * cell_height).min(height) as usize);
        let character_line_list = get_character_line_list_based_on_luma(
            character_data.character_list.clone(),
            img_row,
            cell_width as usize,
        );
        let text_line = character_line_list.join("");

        draw_text_mut(
            &mut out_image,
            Luma([255]),
            0,
            (i * char_height) as i32,
            PxScale {
                x: character_data.scale,
                y: character_data.scale,
            },
            &character_data.font,
            text_line.as_str(),
        );
    }

    return out_image;
}
