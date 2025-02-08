use std::{
    fmt::format,
    fs::{create_dir_all, write},
    path::Path,
};

use crate::{
    character::CharacterType,
    utils::utils::{convert_grayscale_img_to_ndarray, get_character_line_list_based_on_luma},
};
use image::{DynamicImage, GenericImageView};

/// Converts an image to ASCII art.
pub fn image_to_text(
    path: &str,
    num_cols: u32,
    complex: bool,
    output_directory: Option<&str>,
    filename: Option<&str>,
) -> Result<String, String> {
    let img = image::open(path).expect("Failed to open image");
    let gray_img = img.grayscale();

    let ascii = grayscale_to_ascii(&gray_img, num_cols, complex);

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
        let mut path: String = format!("{}.txt", filename.unwrap());
        if output_directory.is_some() {
            path = format!("{}{}", output_directory.unwrap(), filename.unwrap());
        }
        let _ = write(path, ascii.clone());
    }

    return Ok(ascii);
}

fn grayscale_to_ascii(img: &DynamicImage, num_cols: u32, complex: bool) -> String {
    let (width, height) = img.dimensions();
    let mut ascii = String::new();
    let mut num_cols = num_cols;
    let mut cell_width = width / num_cols;
    let mut cell_height = 2 * cell_width;
    let mut num_rows = height / cell_height;
    let character_array: Vec<char> = if complex {
        CharacterType::Complex.get_character_array()
    } else {
        CharacterType::Simple.get_character_array()
    };

    if num_cols > width || num_rows > height {
        // Too many columns or rows. Use default setting
        cell_width = 6;
        cell_height = 12;
        num_cols = width / cell_width;
        num_rows = height / cell_height;
    }

    // Convert img to grayscale ndarray
    let img_array = convert_grayscale_img_to_ndarray(&img);

    for x in 0..num_rows {
        let img_row = img_array.row((x * cell_height) as usize);
        let character_line_list = get_character_line_list_based_on_luma(
            character_array.clone(),
            img_row,
            cell_width as usize,
        );
        let text_line = character_line_list.join("");

        ascii.push_str(&text_line);
        ascii.push_str("\n"); // Newline after each row
    }

    return ascii;
}
