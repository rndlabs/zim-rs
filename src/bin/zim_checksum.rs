use std::{error::Error, process};
use clap::{App, Arg};
use zim::Zim;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("zim_checksum")
        .version("0.1.0")
        .about("Verify the checksum of a ZIM file")
        .author("mfw78")
        .arg(
            Arg::new("input")
                .help("the input file to use")
                .required(true)
                .index(1)
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();

    // run the program
    if let Err(e) = run(input) {
        eprintln!("Checksum verification failed: {}", e);
        process::exit(1);
    }

    Ok(())
}

fn run(input: &str) -> Result<(), Box<dyn Error>> {

    // open the file
    let zim = Zim::new(input)?;
    zim.verify_checksum()?;

    Ok(())
}