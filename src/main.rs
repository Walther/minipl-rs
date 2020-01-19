use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

extern crate clap;
use clap::{App, Arg};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    let matches = App::new("minipl-rs")
        .version("0.1")
        .about("Interpreter for the pedagogic language Mini-PL, for the Compilers course at Univ. Helsinki")
        .author("Veeti 'Walther' Haapsamo")
        .arg("-d, --debug 'Turn debugging information on'")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    if matches.is_present("debug") {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Debug)
            .init()
    }

    if let Some(filename) = matches.value_of("INPUT") {
        debug!("Reading file: {}", filename);
        let _result = read_file(filename);
    }
}

fn read_file(filename: &str) -> Result<(), Error> {
    let file = File::open(filename)?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let lines = contents.lines();
    debug!("Lines: {}", lines.count());

    Ok(())
}
