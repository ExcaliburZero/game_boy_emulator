extern crate clap;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

use clap::{App, Arg, ArgMatches, SubCommand};

use game_boy_emulator::rom;

fn main() {
    let matches = App::new("game_boy_emulator")
        .version("0.1.0")
        .author("Christopher Wells")
        .about("")
        .subcommand(
            SubCommand::with_name("parse_rom")
                .about("")
                .arg(Arg::with_name("ROM").required(true).index(1)),
        )
        .get_matches();

    if let Some(parse_rom_matches) = matches.subcommand_matches("parse_rom") {
        parse_rom(&matches, &parse_rom_matches);
    }
}

fn parse_rom(_matches: &ArgMatches, rom_matches: &ArgMatches) {
    let rom_filepath = rom_matches.value_of("ROM").unwrap();

    let rom_bytes = load_file_bytes(rom_filepath).unwrap();
    let rom = rom::ROM::from_bytes(&rom_bytes).unwrap();

    println!("{}", rom_filepath);
    println!("Title: {}", rom.title);
}

fn load_file_bytes(filepath: &str) -> io::Result<Vec<u8>> {
    let f = File::open(filepath)?;
    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}
