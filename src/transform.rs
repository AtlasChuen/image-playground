extern crate image;
use std::fmt::{self, Formatter, Display};
use image::{GrayImage};


struct Block<'a, T> {
    value: &'a [T; 64]
}

impl <'a>Display for Block<'a, u8> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                write!(f, "{:5.1} ", self.value[i*8 + j])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl <'a>Display for Block<'a, f64> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                write!(f, "{:5.1} ", self.value[i*8 + j])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn display_block(x: u32, y: u32, gray_img: &GrayImage) {
    let mut block = [0f64; 64];
    for i in x..(8 + x) {
        for j in y..(8 + y) {
            block[8 * (i-x) as usize + (j-y) as usize] = gray_img.get_pixel(i as u32, j as u32)[0] as f64;
        }
    }
    println!("{}", Block{value: &block});
}


pub fn normalize(arr : &mut [f64; 64], res : &mut [u8; 64]) {
    for i in 0..64 {
        if arr[i] > 255.0 { res[i] = 255; }
        else if arr[i] < 0.0 { res[i] = 0; }
        else { res[i] = arr[i] as u8; }
    }
}



pub fn compress(coefficients : &mut [f64; 64], thresh: f64) {
  let bench = thresh * coefficients.iter().cloned().fold(0./0., f64::max);
  for i in 0..64 {
      if coefficients[i].abs() < bench {
          coefficients[i] = 0.0;
      }
  }
}