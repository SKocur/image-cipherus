use image::ImageResult;

use crate::command::parser::Configuration;
use crate::processing::algorithms;

pub enum Algorithm {
    LLBE,
}

impl Algorithm {
    pub fn get_all() -> &'static [&'static str] {
        &["LLBE"]
    }
}

pub trait Encoder {
    fn encode(&mut self);

    fn save_image(&self) -> ImageResult<()>;
}

pub trait Decoder {
    fn decode(&mut self) -> String;
}

pub fn get_encoder(config: Configuration) -> Box<Encoder> {
    match config.algorithm {
        Algorithm::LLBE => Box::new(algorithms::llbe::LLBE::new(config)),
    }
}

pub fn get_decoder(config: Configuration) -> Box<Decoder> {
    match config.algorithm {
        Algorithm::LLBE => Box::new(algorithms::llbe::LLBE::new(config)),
    }
}
