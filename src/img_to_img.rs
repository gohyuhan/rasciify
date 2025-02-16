use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgba};
use imageproc::drawing::draw_text_mut;

use crate::{
    character::{CharacterType, FontData},
    utils::{
        font::get_character_dimensions,
        utils::{
            check_and_create_directory, get_character_and_rgba_based_on_rgba,
            get_character_line_list_based_on_luma,
        },
    },
};

// Converts an image to ASCII art.
pub fn image_to_image(
    path: &str,
    num_cols: u32,
    character_type: CharacterType,
    output_directory: Option<&str>,
    filename: Option<&str>,
    is_white_bg: bool,
    is_color: bool,
) -> Result<String, String> {
    let img = image::open(path).expect("Failed to open image");

    // process to generate ascii rgb image or ascii grayscale image
    if is_color {
        // todo: process image to rgb ascii image
        let rgba_ascii_img = rgb_to_rgb_ascii_img(&img, num_cols, character_type, is_white_bg);
        match check_and_create_directory(output_directory) {
            Ok(_) => {
                if let Some(filename) = filename {
                    let path = if let Some(output_dir) = output_directory {
                        format!("{}/{}.png", output_dir, filename)
                    } else {
                        format!("{}.png", filename)
                    };
                    let _ = rgba_ascii_img.save(path);
                    if output_directory.is_some() {
                        return Ok(format!(
                            "Image saved to path: {} as {}.png",
                            output_directory.unwrap(),
                            filename
                        ));
                    } else {
                        return Ok(format!(
                            "Image saved to current directory as {}.png",
                            filename
                        ));
                    }
                }
                return Err("No filename provided".to_string());
            }
            Err(e) => Err(e),
        }
    } else {
        let gray_ascii_img = grayscale_to_ascii_img(&img, num_cols, character_type, is_white_bg);
        match check_and_create_directory(output_directory) {
            Ok(_) => {
                if let Some(filename) = filename {
                    let path = if let Some(output_dir) = output_directory {
                        format!("{}/{}.jpg", output_dir, filename)
                    } else {
                        format!("{}.jpg", filename)
                    };
                    let _ = gray_ascii_img.save(path);
                    if output_directory.is_some() {
                        return Ok(format!(
                            "Image saved to path: {} as {}.jpg",
                            output_directory.unwrap(),
                            filename
                        ));
                    } else {
                        return Ok(format!(
                            "Image saved to current directory as {}.jpg",
                            filename
                        ));
                    }
                }
                return Err("No filename provided".to_string());
            }
            Err(e) => Err(e),
        }
    }
}

pub fn grayscale_to_ascii_img(
    img: &DynamicImage,
    num_cols: u32,
    character_type: CharacterType,
    is_white_bg: bool,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let img = img.grayscale();
    let (width, height) = img.dimensions();
    let character_data: FontData = character_type.get_character_data();
    let mut num_cols = num_cols;
    // width per cell
    let mut cell_width = width / num_cols;
    // height per cell
    let mut cell_height = 2 * cell_width;
    let mut num_rows = height / cell_height;
    let background_code = if is_white_bg { 255 } else { 0 };

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
    let mut out_image: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_pixel(
        output_image_width,
        output_image_height,
        Luma([background_code]),
    );

    // Get the character based on the grayscale value and form a row of characters and draw it on the image row by row
    for i in 0..num_rows {
        let character_line_list = get_character_line_list_based_on_luma(
            character_data.character_list.clone(),
            &img,
            num_cols,
            cell_width,
            width,
            cell_height,
            height,
            i,
        );
        let text_line: String = character_line_list.into_iter().collect();

        draw_text_mut(
            &mut out_image,
            Luma([255 - background_code]),
            0,
            (i * char_height) as i32,
            character_data.scale,
            &character_data.font,
            &text_line,
        );
    }
    return out_image;
}

pub fn rgb_to_rgb_ascii_img(
    img: &DynamicImage,
    num_cols: u32,
    character_type: CharacterType,
    is_white_bg: bool,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let character_data: FontData = character_type.get_character_data();
    let mut num_cols = num_cols;
    // width per cell
    let mut cell_width = width / num_cols;
    // height per cell
    let mut cell_height = 2 * cell_width;
    let mut num_rows = height / cell_height;
    let background_code = if is_white_bg { 255 } else { 0 };

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
    let mut out_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(
        output_image_width,
        output_image_height,
        Rgba([background_code, background_code, background_code, 255]),
    );

    // Get the character based on the mean rgb value and \draw it on the image pixel by pixel
    for i in 0..num_rows {
        for j in 0..num_cols {
            let (character, rgba_value) = get_character_and_rgba_based_on_rgba(
                character_data.character_list.clone(),
                img,
                cell_width,
                width,
                cell_height,
                height,
                i,
                j,
            );

            draw_text_mut(
                &mut out_image,
                rgba_value,
                (j * char_width) as i32,
                (i * char_height) as i32,
                character_data.scale,
                &character_data.font,
                &character.to_string(),
            );
        }
    }
    return out_image;
}
