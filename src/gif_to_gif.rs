use std::{
    fs::File,
    io::{Cursor, Write},
};

use gif::{ColorOutput, DecodeOptions, Decoder, Encoder, Frame, Repeat};
use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgba, RgbaImage};

use rayon::prelude::*;

use crate::{
    grayscale_to_ascii_img, rgb_to_rgb_ascii_img,
    utils::utils::{
        check_and_create_directory, get_img_flatten_gray_and_color_map,
        get_img_flatten_rgb_and_color_map,
    },
    CharacterType,
};

pub fn gif_to_gif(
    path: &str,
    num_cols: u32,
    character_type: CharacterType,
    output_directory: Option<&str>,
    filename: Option<&str>,
    is_white_bg: bool,
    is_color: bool,
) -> Result<String, String> {
    let file = File::open(path).expect("Failed to open GIF");

    // init the decode option for gif
    let options = DecodeOptions::new();
    if is_color {
        let gif_buffer: Cursor<Vec<u8>> =
            rgb_gif_to_ascii_rgb_gif(file, options, num_cols, character_type, is_white_bg);
        match check_and_create_directory(output_directory) {
            Ok(_) => {
                if let Some(filename) = filename {
                    let path = if let Some(output_dir) = output_directory {
                        format!("{}/{}.gif", output_dir, filename)
                    } else {
                        format!("{}.gif", filename)
                    };
                    let mut output_file = File::create(path).unwrap();
                    let _ = output_file.write_all(&gif_buffer.into_inner()).unwrap();
                    if output_directory.is_some() {
                        return Ok(format!(
                            "Gif saved to path: {} as {}.gif",
                            output_directory.unwrap(),
                            filename
                        ));
                    } else {
                        return Ok(format!(
                            "Gif saved to current directory as {}.gif",
                            filename
                        ));
                    }
                }
                return Err("No filename provided".to_string());
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        let gif_buffer: Cursor<Vec<u8>> =
            rgb_gif_to_ascii_grayscale_gif(file, options, num_cols, character_type, is_white_bg);
        match check_and_create_directory(output_directory) {
            Ok(_) => {
                if let Some(filename) = filename {
                    let path = if let Some(output_dir) = output_directory {
                        format!("{}/{}.gif", output_dir, filename)
                    } else {
                        format!("{}.gif", filename)
                    };
                    let mut output_file = File::create(path).unwrap();
                    let _ = output_file.write_all(&gif_buffer.into_inner()).unwrap();
                    if output_directory.is_some() {
                        return Ok(format!(
                            "Gif saved to path: {} as {}.gif",
                            output_directory.unwrap(),
                            filename
                        ));
                    } else {
                        return Ok(format!(
                            "Gif saved to current directory as {}.gif",
                            filename
                        ));
                    }
                }
                return Err("No filename provided".to_string());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

// the function that bundle all the process that need to:
// 1. decode gif to rgba frames
// 2. read the frames and turn frames -> ImageRgba -> DynamicImage -> process image to turn to ascii art in DynamicImage
// 3. i ) based on the list of DynamicImage get color map and flatten the rgba image to turn into paletted image
//    ii) initialize an encoder, write the frames to a buffer and return the buffer
pub fn rgb_gif_to_ascii_rgb_gif(
    gif_file: File,
    options: DecodeOptions,
    num_cols: u32,
    character_type: CharacterType,
    is_white_bg: bool,
) -> Cursor<Vec<u8>> {
    let decoder = decode_gif(gif_file, options, true);

    let rgba_image_buffer_list =
        process_frames_to_ascii_rgba_img(decoder, num_cols, character_type, is_white_bg);

    return encode_images_to_ascii_rgb_gif(&rgba_image_buffer_list);
}

// the function that bundle all the process that need to:
// 1. decode gif to grayscale frames
// 2. read the frames and turn frames -> GrayImage -> DynamicImage -> process image to turn to ascii art in DynamicImage
// 3. i ) based on the list of DynamicImage get color map and flatten the grayscale image to turn into paletted image
//    ii) initialize an encoder, write the frames to a buffer and return the buffer
pub fn rgb_gif_to_ascii_grayscale_gif(
    gif_file: File,
    options: DecodeOptions,
    num_cols: u32,
    character_type: CharacterType,
    is_white_bg: bool,
) -> Cursor<Vec<u8>> {
    let decoder = decode_gif(gif_file, options, false);

    let luma_image_buffer_list =
        process_frames_to_ascii_grayscale_img(decoder, num_cols, character_type, is_white_bg);

    return encode_images_to_ascii_gray_gif(&luma_image_buffer_list);
}

// decode the gif to frames and be process later
pub fn decode_gif(gif_file: File, mut options: DecodeOptions, is_color: bool) -> Decoder<File> {
    // set the decode option to be decoding as RGBA or Indexed
    if is_color {
        options.set_color_output(ColorOutput::RGBA);
    } else {
        options.set_color_output(ColorOutput::Indexed);
    }
    //  decode the file with the set options
    let decoder = options.read_info(gif_file).unwrap();

    return decoder;
}

// encode the rgba ascii art images back to rgb frames and return the buffer
pub fn encode_images_to_ascii_rgb_gif(
    rgba_image_buffer_list: &Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
) -> Cursor<Vec<u8>> {
    let mut gif_buffer = Cursor::new(Vec::<u8>::new());
    let mut encoder_width: u16 = 0;
    let mut encoder_height: u16 = 0;

    let (flatten_rgb, color_map) = get_img_flatten_rgb_and_color_map(rgba_image_buffer_list);
    rgba_image_buffer_list.iter().for_each(|frame| {
        encoder_height = encoder_height.max(frame.height() as u16);
        encoder_width = encoder_width.max(frame.width() as u16);
    });

    // start the encoding process
    let mut encoder =
        Encoder::new(&mut gif_buffer, encoder_width, encoder_height, &color_map).unwrap();
    let _ = encoder.set_repeat(Repeat::Infinite);

    // get the rgb gif frame from the flatten rgb, dynamic image is neede to get the width and height info as the flattern rgb is just a 1d array
    let rgb_gif_frame = get_rgb_gif_frame(rgba_image_buffer_list, &flatten_rgb);

    // Convert images and write frames to GIF
    rgb_gif_frame.iter().for_each(|frame| {
        encoder.write_frame(frame).unwrap();
    });

    drop(encoder);

    return gif_buffer.clone();
}

// encode the gray ascii art images back to gray frames and return the buffer
pub fn encode_images_to_ascii_gray_gif(
    luma_image_buffer_list: &Vec<ImageBuffer<Luma<u8>, Vec<u8>>>,
) -> Cursor<Vec<u8>> {
    let mut gif_buffer = Cursor::new(Vec::<u8>::new());
    let mut encoder_width: u16 = 0;
    let mut encoder_height: u16 = 0;

    let (flatten_gray, color_map) = get_img_flatten_gray_and_color_map(luma_image_buffer_list);
    luma_image_buffer_list.iter().for_each(|frame| {
        encoder_height = encoder_height.max(frame.height() as u16);
        encoder_width = encoder_width.max(frame.width() as u16);
    });

    // start the encoding process
    let mut encoder =
        Encoder::new(&mut gif_buffer, encoder_width, encoder_height, &color_map).unwrap();
    let _ = encoder.set_repeat(Repeat::Infinite);

    // get the grayScale gif frame from the flatten gray, dynamic image is neede to get the width and height info as the flattern rgb is just a 1d array
    let grayscale_gif_frame = get_grayscale_gif_frame(luma_image_buffer_list, &flatten_gray);

    // Convert images and write frames to GIF
    grayscale_gif_frame.iter().for_each(|frame| {
        encoder.write_frame(&frame).unwrap();
    });

    drop(encoder);

    return gif_buffer.clone();
}

// ***************************************************************************************
//
//    The following functions utilize rayon parallel processing to speed up the process
//
//    Include process:
//      - process gif frame to ascii art
//      - converting dynamic image to gif compatible frame (both rgb and grayscale) [in future when resizing is supported]
//
// ***************************************************************************************

// process the frames to list of rgba ascii art
pub fn process_frames_to_ascii_rgba_img(
    mut decoder: Decoder<File>,
    num_cols: u32,
    character_type: CharacterType,
    is_white_bg: bool,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut all_frames = vec![];

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        // get all the frame
        all_frames.push(frame.clone());
    }

    // processed rgba image buffer that will later be encoded back to gif
    let rgba_image_buffer_list: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = all_frames
        .par_iter()
        .map(|frame| {
            // Process every frame

            // Convert Frame to Dynamic Image
            let height = frame.height as u32;
            let width = frame.width as u32;

            // read the frame buffer as ImageBuffer of rgba
            let rgb_img_buffer = RgbaImage::from_raw(width, height, frame.buffer.to_vec())
                .expect("Failed to create RgbaImage from frame");

            // convert to rgba dynamic image ffrom the buffer
            let rgb_img = DynamicImage::ImageRgba8(rgb_img_buffer);

            let rgb_ascii_img =
                rgb_to_rgb_ascii_img(&rgb_img, num_cols, character_type, is_white_bg);

            rgb_ascii_img
        })
        .collect();

    return rgba_image_buffer_list;
}

// process the frames to list of gray ascii art
pub fn process_frames_to_ascii_grayscale_img(
    mut decoder: Decoder<File>,
    num_cols: u32,
    character_type: CharacterType,
    is_white_bg: bool,
) -> Vec<ImageBuffer<Luma<u8>, Vec<u8>>> {
    let mut all_frames = vec![];

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        // get all the frame
        all_frames.push(frame.clone());
    }

    // processed luma8 image buffer that will later be encoded back to gif
    let luma8_image_buffer_list: Vec<ImageBuffer<Luma<u8>, Vec<u8>>> = all_frames
        .par_iter()
        .map(|frame| {
            // Process every frame

            // Convert Frame to Dynamic Image
            let height = frame.height as u32;
            let width = frame.width as u32;

            // read the frame buffer as ImageBuffer of luma8
            let luma_img_buffer = GrayImage::from_raw(width, height, frame.buffer.to_vec())
                .expect("Failed to create GrayImage from frame");

            // convert to luma8 dynamic image from the buffer
            let luma_img = DynamicImage::ImageLuma8(luma_img_buffer);

            let luma_ascii_img =
                grayscale_to_ascii_img(&luma_img, num_cols, character_type, is_white_bg);

            luma_ascii_img
        })
        .collect();
    return luma8_image_buffer_list;
}

// a function to convert the flatten rgb to gif frame
// utilizing rayon parallel processing to faster the process
fn get_rgb_gif_frame(
    dynamic_image_list: &Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    flatten_rgb: &Vec<Vec<u8>>,
) -> Vec<Frame<'static>> {
    let frame_array: Vec<Frame<'static>> = dynamic_image_list
        .par_iter()
        .enumerate()
        .map(|(index, img)| {
            let pixel = flatten_rgb[index].clone(); // Cloning to own the data
            let width = img.width() as u16;
            let height = img.height() as u16;
            let frame = Frame::from_rgb_speed(width, height, &pixel, 10);
            frame
        })
        .collect();

    return frame_array;
}

// a function to convert the flatten gray to gif frame
// utilizing rayon parallel processing to faster the process
fn get_grayscale_gif_frame(
    dynamic_image_list: &Vec<ImageBuffer<Luma<u8>, Vec<u8>>>,
    flatten_gray: &Vec<Vec<u8>>,
) -> Vec<Frame<'static>> {
    let frame_array: Vec<Frame<'static>> = dynamic_image_list
        .par_iter()
        .enumerate()
        .map(|(index, img)| {
            let mut pixel = flatten_gray[index].clone(); // Cloning to own the data
            let width = img.width() as u16;
            let height = img.height() as u16;
            // it will be quite fast for rgb that was originally grayscale
            // so we will not use Frame::from_rgb_speed like the rgb one does
            let frame = Frame::from_rgb(width, height, &mut pixel);
            frame
        })
        .collect();

    return frame_array;
}
