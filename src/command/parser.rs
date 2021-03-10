use clap::ArgMatches;

use crate::processing::algorithm::Algorithm;

pub enum Mode {
    ENCODING,
    DECODING,
}

pub struct Configuration {
    pub image_path: String,
    pub mode: Mode,
    pub text_to_encrypt: String,
    pub algorithm: Algorithm,
}

pub fn parse_args(args: ArgMatches) -> Configuration {
    let mode: Mode;
    if let Some(tmp) = args.value_of("mode") {
        mode = match tmp {
            "enc" => Mode::ENCODING,
            "dec" => Mode::DECODING,
            _ => panic!("Wrong mode provided"),
        }
    } else {
        panic!("mode is not provided")
    }

    let mut text_to_encrypt: String = String::new();
    if let Some(tmp) = args.value_of("data") {
        text_to_encrypt = String::from(tmp);
    }

    let img: String;

    if let Some(tmp) = args.value_of("file") {
        img = String::from(tmp);
    } else {
        panic!("file name is not present")
    }

    return Configuration {
        image_path: img,
        mode: mode,
        text_to_encrypt: text_to_encrypt,
        algorithm: Algorithm::LLBE,
    };
}
