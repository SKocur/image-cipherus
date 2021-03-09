use image::{DynamicImage, GenericImage, GenericImageView, ImageResult, Rgba, ImageBuffer};
use image::buffer::EnumeratePixelsMut;

use crate::command::parser::Configuration;
use crate::processing::algorithm::{Decoder, Encoder};

pub struct LLBE {
    config: Configuration,
    image: DynamicImage,
    imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    col: u32,
    row: u32,
}

/**
 * LLBE - Low Level Bit Encryption
 */
impl LLBE {
    pub fn new(config: Configuration) -> Self {
        let img = image::open(&config.image_path).unwrap();
        Self {
            imgbuf: img.to_rgba(),
            image: img,
            config: config,
            col: 0,
            row: 0,
        }
    }

    fn encode_byte(&mut self, ch: char) {
        for i in 0..8 {
            self.encode_bit_character((ch as u8 >> i) & 0b1)
        }
    }

    fn encode_bit_character(&mut self, diff: u8) {
        if self.col < self.image.width() - 1 {
            // let mut pixel: Rgba<u8> = self.image.get_pixel(self.col, self.row);
            let pixel = self.imgbuf.get_pixel_mut(self.col, self.row);
            let rgba = pixel.0;

            let next_pixel = self.image.get_pixel(self.col + 1, self.row);
            let mut b = next_pixel.0[2];
            b -= diff;

            *pixel = Rgba([rgba[0], rgba[1], b, rgba[3]]);

            self.col += 2;
        } else {
            self.row += 1;
            self.col = 0;
        }
    }

    fn decrypt_bit_character(&mut self) -> u32 {
        if self.col < self.image.width() - 1 {
            let b = self.image.get_pixel(self.col, self.row).0[2] as i32;
            
            let new_b = self.image.get_pixel(self.col + 1, self.row).0[2] as i32;

            self.col += 2;

            ((new_b - b) as u32)
        } else {
            self.row += 1;
            self.col = 0;

            1
        }
    }
}

impl Encoder for LLBE {
    fn encode(&mut self) {
        println!("Encoding using LLBE");
        let chars: Vec<char> = self.config.text_to_encrypt.chars().collect();
        for chr in chars {
            self.encode_byte(chr);
        }

        self.col = 0;
        self.row = 0;
    }

    fn save_image(&self) -> ImageResult<()> {
        self.imgbuf.save("result.png")
    }
}

impl Decoder for LLBE {
    fn decode(&mut self) -> String {
        println!("Decoding using LLBE");

        let mut res = String::new();

        while self.col < self.image.width() - 1 && self.row < self.image.height() {
            let mut c: u32 = 0;

            for _ in 0..8 {
                c <<= 1;
                c |= self.decrypt_bit_character();
            }

            let chr = std::char::from_u32((c << 24).reverse_bits() & 0b11111111);

            match chr {
                Some(c) => {
                    // print!("{}\t", c);
                    res.push_str(&c.to_string())
                },
                None => continue
            }
        }
        return res;
    }
}
