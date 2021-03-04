use clap::ArgMatches;
use image::DynamicImage;

pub enum Mode {
    ENCRYPTING,
    DECRYPTING,
}

pub struct Configuration {
    image: DynamicImage,
    mode: Mode,
    text_to_encrypt: String,
}

pub fn parse_args(args: ArgMatches) -> Configuration {
    let img: DynamicImage;

    if let Some(tmp) = args.value_of("file") {
        img = image::open(tmp).unwrap();
    } else {
        panic!("file name is not present")
    }

    let mode: Mode;
    if let Some(tmp) = args.value_of("mode") {
        mode = match tmp {
            "enc" => Mode::ENCRYPTING,
            "dec" => Mode::DECRYPTING,
            _ => panic!("Wrong mode provided"),
        }
    } else {
        panic!("mode is not provided")
    }

    let mut text_to_encrypt: String = String::new();
    if let Some(tmp) = args.value_of("data") {
        text_to_encrypt = String::from(tmp);
    }

    return Configuration {
        image: img,
        mode: mode,
        text_to_encrypt: text_to_encrypt,
    };
}
