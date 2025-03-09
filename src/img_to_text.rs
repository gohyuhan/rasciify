use std::fs::write;

use crate::{
    character::CharacterType,
    utils::utils::{check_and_create_directory, get_character_line_list_based_on_luma},
};
use image::{DynamicImage, GenericImageView};

// Converts an image to ASCII art.
pub fn image_to_text(
    path: &str,
    num_cols: u32,
    complex: bool,
    output_directory: Option<&str>,
    filename: Option<&str>,
) -> Result<String, String> {
    let img = image::open(path).expect("Failed to open image");

    let ascii = grayscale_to_ascii(&img, num_cols, complex);

    match check_and_create_directory(output_directory) {
        Ok(_) => {
            if let Some(filename) = filename {
                let mut path: String = format!("{}.txt", filename);
                if output_directory.is_some() {
                    path = format!("{}{}", output_directory.unwrap(), filename);
                }
                let _ = write(path, ascii.clone());
            }
            return Ok(ascii);
        }
        Err(e) => Err(e),
    }
}

pub fn grayscale_to_ascii(img: &DynamicImage, num_cols: u32, complex: bool) -> String {
    let img = img.grayscale();
    let (width, height) = img.dimensions();
    let mut ascii = String::new();
    let mut num_cols = if num_cols > width { width } else { num_cols };
    let mut cell_width = width / num_cols;
    let mut cell_height = 2 * cell_width;
    let mut num_rows = height / cell_height;
    let character_array: Vec<char> = if complex {
        CharacterType::Complex.get_character_data().character_list
    } else {
        CharacterType::Simple.get_character_data().character_list
    };

    if num_cols > width || num_rows > height {
        // Too many columns or rows. Use default setting
        cell_width = 6;
        cell_height = 12;
        num_cols = width / cell_width;
        num_rows = height / cell_height;
    }

    for x in 0..num_rows {
        let character_line_list = get_character_line_list_based_on_luma(
            character_array.clone(),
            &img,
            num_cols,
            cell_width,
            width,
            cell_height,
            height,
            x,
        );
        let text_line: String = character_line_list.into_iter().collect();

        ascii.push_str(&text_line);
        ascii.push_str("\n"); // Newline after each row
    }

    return ascii;
}
