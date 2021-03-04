extern crate clap;
extern crate image;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Image Cipherus")
        .version("0.1.0")
        .author("Szymon Kocur <skocur10@gmail.com>")
        .about("Command-line application that helps in encrypting data into image and decrypting data from imag.")
        .get_matches();
        
}
