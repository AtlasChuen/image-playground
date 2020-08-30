use wasm_bindgen::prelude::*;

extern crate image;

// use image::{ImageFormat, GenericImageView, RgbImage, ImageBuffer, GrayImage, Luma};
use image::{ImageFormat, GenericImageView, RgbImage, Luma};

use std::fs::File;
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;

mod transform;
mod dct;

fn visit_block(mut vec: &mut [f64; 64], mut res : &mut [u8; 64]) {
    dct::dct(&mut vec);
    // println!("{}", Block{value: &vec});
    transform::compress(&mut vec, 0.012);
    dct::idct(&mut vec);
    transform::normalize(vec, &mut res);
}

#[wasm_bindgen]
pub fn grayscalify(buffer: &[u8]) {
    let img = image::load_from_memory(buffer).unwrap();
    _initialize();
    let mut output = File::create("/uploaded/uploaded.jpg").unwrap();
    img.grayscale().write_to(&mut output, ImageFormat::Jpeg).unwrap();
}


#[wasm_bindgen]
pub fn jpeg_compress(buffer: &[u8]) -> Vec<u8>{
    let img = image::load_from_memory(buffer).unwrap();

    println!("dimensions {:?}", img.dimensions());
    let width = img.dimensions().0;
    let height = img.dimensions().1;
    let mut gray_img = img.into_luma();
    
    transform::display_block(312, 200, &gray_img);
    
    for w in 0..width/8 {
        for h in 0..height/8 {
            let mut block = [0f64; 64];
            for x in 0..8 {
                for y in 0..8 {
                    block[8 * x + y] = gray_img.get_pixel(w * 8 + x as u32, h * 8 + y as u32)[0] as f64;
                }
            }
            let mut res = [0u8; 64];
            visit_block(&mut block, &mut res);
            for x in 0..8 {
                for y in 0..8 {
                    gray_img.put_pixel(w * 8 + x as u32, h * 8 + y as u32, Luma([res[8 * x + y]]));
                }
            }
        }
    }
    transform::display_block(312, 200, &gray_img);
    
    // gray_img.save("gray.jpg").unwrap();
    // let mut output = File::create("/uploaded/test.jpg").unwrap();
    // let rgb:RgbImage = gray_img;
    // img.grayscale().write_to(&mut output, ImageFormat::Jpeg).unwrap();
    return gray_img.to_vec();
    // return buffer.to_vec();
}
