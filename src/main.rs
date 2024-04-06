extern crate image;

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //input
    let img = image::open("input.jpg").unwrap(); // this is your input, you are need to rename file in directory if this app

    let gray_img = img.to_luma8();
    
    let contrast_img = adjust_contrast(&gray_img, 0.12); //choose contrast (set default if you don't know what you do)

    let (width, height) = (1400, 1066); //adjust proportions like in your picture

    let resized_img = image::imageops::resize(&contrast_img, width, height, image::imageops::FilterType::Nearest);

    let resized_dynamic_img = DynamicImage::ImageLuma8(resized_img);

    let ascii_art = generate_ascii_art(&resized_dynamic_img);

    //output
    let mut file = File::create("output.txt").expect("Unable to create file");
    file.write_all(ascii_art.as_bytes()).expect("Unable to write data to file");
}

// Function to adjust the contrast of an image
fn adjust_contrast(image: &ImageBuffer<Luma<u8>, Vec<u8>>, factor: f32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let mut new_image = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let intensity = pixel[0] as f32 / 255.0;

            let adjusted_intensity = intensity * factor;
            let new_pixel_value = (adjusted_intensity * 255.0).round() as u8;

            new_image.put_pixel(x, y, Luma([new_pixel_value]));
        }
    }

    new_image
}

// Function to generate ASCII art from an image
fn generate_ascii_art(image: &DynamicImage) -> String {
    let (width, height) = image.dimensions();
    let mut ascii_art = String::new();

    let ascii_chars = [
        'X', 'N', 'K', 'O', '0', 'k', 'x', 'o', ',', '.', ':', ';', 'l', 'd', 'c', 'h', 'w', 'I', 'J', '!', 'i', ';', ' ' // customized symbols based on provided fragment
    ];

    let aspect_ratio = 0.5; 

    let adjusted_height = (height as f32 * aspect_ratio) as u32;

    for y in 0..adjusted_height {
        for x in 0..width {
            let pixel = image.get_pixel(x, (y as f32 / aspect_ratio) as u32);
            let intensity = pixel[0] as f32 / 255.0;

            let index = ((1.0 - intensity) * (ascii_chars.len() - 1) as f32).round() as usize;
            let ascii_char = ascii_chars[index];

            ascii_art.push(ascii_char);
        }
        ascii_art.push('\n');
    }

    ascii_art
}
