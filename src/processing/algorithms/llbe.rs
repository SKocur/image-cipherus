use image::{ImageBuffer, ImageResult, Rgba};

use crate::command::parser::Configuration;
use crate::processing::algorithm::{Decoder, Encoder};

pub struct LLBE {
    config: Configuration,
    imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    col: u32,
    row: u32,
    temp_bit_val: u32,
}

/**
 * LLBE - Low Level Bit Encryption
 */
impl LLBE {
    pub fn new(config: Configuration) -> Self {
        let img = image::open(&config.image_path).unwrap();
        Self {
            imgbuf: img.to_rgba8(),
            config: config,
            col: 0,
            row: 0,
            temp_bit_val: 0,
        }
    }

    fn encode_byte(&mut self, ch: char) {
        for i in 0..8 {
            self.encode_bit_character((ch as u8 >> i) & 0b1)
        }
    }

    fn encode_bit_character(&mut self, diff: u8) {
        if self.col >= self.imgbuf.width() - 1 && self.row < self.imgbuf.height() - 1 {
            self.row += 1;
            self.col = 0;
        }

        let next_pixel = self.imgbuf.get_pixel(self.col + 1, self.row);
        let mut b = next_pixel.0[2];
        b -= diff;

        let pixel = self.imgbuf.get_pixel_mut(self.col, self.row);
        let rgba = pixel.0;

        *pixel = Rgba([rgba[0], rgba[1], b, rgba[3]]);

        self.col += 2;
    }

    fn decode_bit_character(&mut self) {
        if self.col < self.imgbuf.width() - 1 {
            let b = self.imgbuf.get_pixel(self.col, self.row).0[2] as i32;

            let new_b = self.imgbuf.get_pixel(self.col + 1, self.row).0[2] as i32;

            self.col += 2;

            self.temp_bit_val = (new_b - b) as u32;
        } else if self.col >= self.imgbuf.width() - 1 && self.row < self.imgbuf.height() - 1 {
            self.row += 1;
            self.col = 0;

            let b = self.imgbuf.get_pixel(self.col, self.row).0[2] as i32;

            let new_b = self.imgbuf.get_pixel(self.col + 1, self.row).0[2] as i32;
            
            self.col += 2;

            self.temp_bit_val = (new_b - b) as u32;
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

        while self.col < self.imgbuf.width() - 1 && self.row < self.imgbuf.height() {
            let mut c: u32 = 0;

            for _ in 0..8 {
                self.decode_bit_character();

                c <<= 1;
                c |= self.temp_bit_val;
            }

            let tmp_res = (c << 24).reverse_bits() & 0b11111111;

            // Trial and error shows that this threshold is perfect for ignoring image noise
            if tmp_res > 200 {
                break;
            }

            let chr = std::char::from_u32(tmp_res);

            match chr {
                Some(c) => res.push_str(&c.to_string()),
                None => continue,
            }
        }
        return res.trim_matches(char::from(0)).to_string();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::command::parser::Mode;
    use crate::processing::algorithm::Algorithm;

    #[test]
    fn test_encoding_single_letter() {
        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(16, 1);
        imgbuf.put_pixel(0, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(1, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(2, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(3, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(4, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(5, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(6, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(7, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(8, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(9, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(10, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(11, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(12, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(13, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(14, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(15, 0, Rgba([0, 0, 1, 0xFF]));

        let mut algo = LLBE {
            imgbuf: imgbuf,
            col: 0,
            row: 0,
            temp_bit_val: 0,
            config: Configuration {
                algorithm: Algorithm::LLBE,
                mode: Mode::ENCODING,
                image_path: String::from(""),
                text_to_encrypt: String::from("A"),
            },
        };

        algo.encode();

        assert_eq!(algo.imgbuf.get_pixel(0, 0).0[2], 0);
        assert_eq!(algo.imgbuf.get_pixel(12, 0).0[2], 0);
    }

    #[test]
    fn test_encoding_one_letter_in_two_lines() {
        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(14, 2);
        imgbuf.put_pixel(0, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(1, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(2, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(3, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(4, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(5, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(6, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(7, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(8, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(9, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(10, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(11, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(12, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(13, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(0, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(1, 1, Rgba([0, 0, 1, 0xFF]));

        let mut algo = LLBE {
            imgbuf: imgbuf,
            col: 0,
            row: 0,
            temp_bit_val: 0,
            config: Configuration {
                algorithm: Algorithm::LLBE,
                mode: Mode::ENCODING,
                image_path: String::from(""),
                text_to_encrypt: String::from("A"),
            },
        };

        algo.encode();

        assert_eq!(algo.imgbuf.get_pixel(0, 0).0[2], 0);
        assert_eq!(algo.imgbuf.get_pixel(12, 0).0[2], 0);
    }

    #[test]
    fn test_encoding_two_letters_wrapped_in_three_lines() {
        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(14, 3);
        //first A
        imgbuf.put_pixel(0, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(1, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(2, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(3, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(4, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(5, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(6, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(7, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(8, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(9, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(10, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(11, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(12, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(13, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(0, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(1, 1, Rgba([0, 0, 1, 0xFF]));

        //second A
        imgbuf.put_pixel(2, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(3, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(4, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(5, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(6, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(7, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(8, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(9, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(10, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(11, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(12, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(13, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(0, 2, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(1, 2, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(2, 2, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(3, 2, Rgba([0, 0, 1, 0xFF]));

        let mut algo = LLBE {
            imgbuf: imgbuf,
            col: 0,
            row: 0,
            temp_bit_val: 0,
            config: Configuration {
                algorithm: Algorithm::LLBE,
                mode: Mode::ENCODING,
                image_path: String::from(""),
                text_to_encrypt: String::from("AA"),
            },
        };

        algo.encode();

        assert_eq!(algo.imgbuf.get_pixel(0, 0).0[2], 0);
        assert_eq!(algo.imgbuf.get_pixel(12, 0).0[2], 0);
        assert_eq!(algo.imgbuf.get_pixel(2, 1).0[2], 0);
        assert_eq!(algo.imgbuf.get_pixel(0, 2).0[2], 0);
    }

    #[test]
    fn test_decoding_one_line_single_letter() {
        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(16, 1);
        imgbuf.put_pixel(0, 0, Rgba([0, 0, 0, 0xFF]));
        imgbuf.put_pixel(1, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(2, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(3, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(4, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(5, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(6, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(7, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(8, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(9, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(10, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(11, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(12, 0, Rgba([0, 0, 0, 0xFF]));
        imgbuf.put_pixel(13, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(14, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(15, 0, Rgba([0, 0, 1, 0xFF]));

        let mut algo = LLBE {
            imgbuf: imgbuf,
            col: 0,
            row: 0,
            temp_bit_val: 0,
            config: Configuration {
                algorithm: Algorithm::LLBE,
                mode: Mode::DECODING,
                image_path: String::from("tmp"),
                text_to_encrypt: String::from(""),
            },
        };

        assert_eq!(algo.decode(), "A");
    }

    #[test]
    fn test_decoding_wrapped_two_lines_single_letter() {
        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(14, 2);
        imgbuf.put_pixel(0, 0, Rgba([0, 0, 0, 0xFF]));
        imgbuf.put_pixel(1, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(2, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(3, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(4, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(5, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(6, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(7, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(8, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(9, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(10, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(11, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(12, 0, Rgba([0, 0, 0, 0xFF]));
        imgbuf.put_pixel(13, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(0, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(1, 1, Rgba([0, 0, 1, 0xFF]));

        let mut algo = LLBE {
            imgbuf: imgbuf,
            col: 0,
            row: 0,
            temp_bit_val: 0,
            config: Configuration {
                algorithm: Algorithm::LLBE,
                mode: Mode::DECODING,
                image_path: String::from("tmp"),
                text_to_encrypt: String::from(""),
            },
        };

        assert_eq!(algo.decode(), "A");
    }

    #[test]
    fn test_decoding_wrapped_three_lines_two_letters() {
        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(14, 3);
        //first A
        imgbuf.put_pixel(0, 0, Rgba([0, 0, 0, 0xFF]));
        imgbuf.put_pixel(1, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(2, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(3, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(4, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(5, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(6, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(7, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(8, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(9, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(10, 0, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(11, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(12, 0, Rgba([0, 0, 0, 0xFF]));
        imgbuf.put_pixel(13, 0, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(0, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(1, 1, Rgba([0, 0, 1, 0xFF]));

        //second A
        imgbuf.put_pixel(2, 1, Rgba([0, 0, 0, 0xFF]));
        imgbuf.put_pixel(3, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(4, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(5, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(6, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(7, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(8, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(9, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(10, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(11, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(12, 1, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(13, 1, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(0, 2, Rgba([0, 0, 0, 0xFF]));
        imgbuf.put_pixel(1, 2, Rgba([0, 0, 1, 0xFF]));

        imgbuf.put_pixel(2, 2, Rgba([0, 0, 1, 0xFF]));
        imgbuf.put_pixel(3, 2, Rgba([0, 0, 1, 0xFF]));

        let mut algo = LLBE {
            imgbuf: imgbuf,
            col: 0,
            row: 0,
            temp_bit_val: 0,
            config: Configuration {
                algorithm: Algorithm::LLBE,
                mode: Mode::DECODING,
                image_path: String::from("tmp"),
                text_to_encrypt: String::from(""),
            },
        };

        assert_eq!(algo.decode(), "AA");
    }
}
