use clap::{App, Arg};
use image::ImageResult;

mod command;
mod processing;

use command::parser::{parse_args, Configuration, Mode};
use processing::algorithm::{get_decoder, get_encoder, Algorithm};

fn main() {
    let matches = App::new("Image Cipherus")
        .version("0.1.0")
        .author("Szymon Kocur <skocur10@gmail.com>")
        .about("Command-line application that helps in encrypting data into image and decrypting data from image.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .value_name("file_name")
        )
        .arg(
            Arg::with_name("data")
                .short("d")
                .long("data")
                .takes_value(true)
                .required(false)
                .value_name("text to be encrypted")
        )
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .takes_value(true)
                .required(true)
                .value_name("mode")
                .possible_values(&["enc", "dec"])
        )
        .arg(
            Arg::with_name("algorithm")
            .short("a")
            .long("algo")
            .takes_value(true)
            .required(true)
            .value_name("algorithm")
            .possible_values(Algorithm::get_all())
        )
        .get_matches();

    let config = parse_args(matches);

    match config.mode {
        Mode::ENCODING => encode(config),
        Mode::DECODING => println!("{}", decode(config)),
    }
}

fn encode(config: Configuration) {
    let mut algo = get_encoder(config);
    algo.encode();
    let res: ImageResult<()> = algo.save_image();

    match res {
        Err(e) => eprintln!("Error while saving encoded image {}", e),
        Ok(_) => println!("\x1b[0;32mEncoded image saved successfully\x1b[0m"),
    }
}

fn decode(config: Configuration) -> String {
    let mut alg = get_decoder(config);
    alg.decode()
}
