use std::{
    fs::{
        write,
        create_dir_all
    },
    path::Path,
};

use image::{DynamicImage, GenericImageView};

const ASCII_CHARS: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

/// Converts an image to ASCII art.
pub fn image_to_text(
    path: &str,
    num_cols: u32,
    output_directory: Option<&str>,
    filename: Option<&str>,
) -> Result<String, String> {
    let img = image::open(path).expect("Failed to open image");
    let gray_img = img.grayscale();

    let ascii = grayscale_to_ascii(&gray_img, num_cols);

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
        let mut path:String = format!("{}.txt", filename.unwrap());
        if output_directory.is_some(){
            path = format!("{}{}", output_directory.unwrap(), filename.unwrap());
        }
        let _ = write(path, ascii.clone());
    }

    return Ok(ascii);
}

fn grayscale_to_ascii(img: &DynamicImage, num_cols: u32) -> String {
    let (width, height) = img.dimensions();
    let mut ascii = String::new();
    let mut num_cols = num_cols;
    let mut cell_width = width / num_cols;
    let mut cell_height = 2 * cell_width;
    let mut num_rows = height / cell_height;

    if num_cols > width || num_rows > height {
        // Too many columns or rows. Use default setting
        cell_width = 6;
        cell_height = 12;
        num_cols = width / cell_width;
        num_rows = height / cell_height;
    }

    for x in 0..num_rows {
        for y in 0..num_cols {
            let pixel = img.get_pixel((y * cell_width).min(width), (x * cell_height).min(height));
            let luma = pixel[0]; // Get the grayscale value (0-255)

            // Normalize the grayscale value to the range [0, 1]
            let normalized_luma = luma as f32 / 255.0;

            // Calculate the index into the ASCII character set.
            // Improved index calculation: scale and round.
            let index = (normalized_luma * (ASCII_CHARS.len() - 1) as f32).round() as usize;

            ascii.push(ASCII_CHARS[index]);
        }
        ascii.push('\n'); // Newline after each row
    }

    return ascii;
}
