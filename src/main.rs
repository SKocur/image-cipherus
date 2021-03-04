use clap::{App, Arg, SubCommand};

mod command;

use command::parser;

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
        .get_matches();

    parser::parse_args(matches);
}
