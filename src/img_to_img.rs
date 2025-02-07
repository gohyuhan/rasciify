use std::{
    fs::{
        write,
        create_dir_all
    },
    path::Path,
};

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};
use imageproc::drawing::draw_text_mut;

use crate::{character::CharacterType, utils::font::{get_character_dimensions, get_fonts, FontData}};

/// Converts an image to ASCII art.
pub fn image_to_image(
    path: &str,
    num_cols: u32,
    character_type:CharacterType,
    output_directory: Option<&str>,
    filename: Option<&str>,
) -> Result<String, String> {
    let img = image::open(path).expect("Failed to open image");
    let gray_img = img.grayscale();

    let ascii = grayscale_to_ascii_img(&gray_img, num_cols, character_type);

    // if output_directory.is_some() {
    //     let path = Path::new(output_directory.unwrap());

    //     if path.exists() {
    //         if !path.is_dir() {
    //             return Err(format!(
    //                 "Path exists but is not a directory: {}",
    //                 output_directory.unwrap()
    //             ));
    //         }
    //     } else {
    //         let _ = create_dir_all(path);
    //     }
    // }

    // if filename.is_some() {
    //     let mut path:String = format!("{}.jpg", filename.unwrap());
    //     if output_directory.is_some(){
    //         path = format!("{}{}", output_directory.unwrap(), filename.unwrap());
    //     }
    //     let _ = write(path, ascii);
    // }

    return Ok("Done".to_string());
}

fn grayscale_to_ascii_img(img: &DynamicImage, num_cols: u32, character_type:CharacterType) -> String {
    let (width, height) = img.dimensions();
    let character_data:FontData = get_fonts(character_type).unwrap();
    println!("OG dimension width: {}, height: {}", width, height);
    let mut num_cols = num_cols;
    let mut cell_width = width / num_cols;
    let mut cell_height = 2 * cell_width;
    let mut num_rows = height / cell_height;
    println!("cell width: {} cell height:{}", cell_width, cell_height);

    if num_cols > width || num_rows > height {
        // Too many columns or rows. Use default setting
        cell_width = 6;
        cell_height = 12;
        num_cols = width / cell_width;
        num_rows = height / cell_height;
    }

    let (char_width , char_height) = get_character_dimensions(&character_data.font, character_data.scale, character_data.character);
    println!("char width: {} char height:{}", char_width, char_height);
    let output_image_width =  character_data.size * char_width * num_cols;
    let output_image_height = character_data.size * char_height * num_rows;
    println!("num cols: {} num rows: {}", num_cols, num_rows);
    println!("{}x{}", output_image_width, output_image_height);


    let mut out_image: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(output_image_width, output_image_height);

    // Fill the background with the specified color
    for pixel in out_image.pixels_mut() {
        *pixel = Luma([255]);
    }
    for i in 0..num_rows {
        let mut text_line = "".to_string();
        for j in 0..num_cols{
            let pixel = img.get_pixel((j * cell_width).min(width), (i * cell_height).min(height));
            let luma = pixel[0]; // Get the grayscale value (0-255)

            // Normalize the grayscale value to the range [0, 1]
            let normalized_luma = luma as f32 / 255.0;

            // Calculate the index into the ASCII character set.
            let index = (normalized_luma * (character_data.character_list.len() - 1) as f32).round() as usize;
            text_line = format!("{}{}", text_line, character_data.character_list[index]);
        }
        draw_text_mut(&mut out_image, Luma([0]), 0, (i * char_height * character_data.size) as i32, character_data.scale, &character_data.font , text_line.as_str());
    }

    out_image.save("ascii_text.jpg").unwrap();
    
    return "done".to_string();
}