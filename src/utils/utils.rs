use image::{DynamicImage, GenericImageView};
use ndarray::{Array2, ArrayView1};

pub fn convert_grayscale_img_to_ndarray(image: &DynamicImage) -> Array2<u8> {
    let (width, height) = image.dimensions();
    let pixels_grayscale_luma_array = image.pixels().map(|p| p.2[0]).collect();

    let img_array = Array2::from_shape_vec(
        (height as usize, width as usize),
        pixels_grayscale_luma_array,
    )
    .unwrap();

    return img_array;
}

pub fn get_character_line_list_based_on_luma(
    character_list: Vec<char>,
    luma_list: ArrayView1<u8>,
    step: usize,
) -> Vec<String> {
    let character_line_list: Vec<String> = luma_list
        .iter()
        .step_by(step)
        .map(|&luma| {
            let index = (luma as f32 / 255.0 * (character_list.len() - 1) as f32).round() as usize;
            String::from(character_list[index])
        })
        .collect();

    return character_line_list;
}
