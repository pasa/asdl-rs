use std::fs;
use std::path::Path;

use clap::{App, Arg};

use asdl_rs::{generate, Result};

fn main() -> Result<()> {
    let matches = App::new("Asdl generator")
        .version("0.1.0")
        .author("Sergey Parilin <parilinsa@gmail.com>")
        .about("Parses asdl notation and generates source files according template.")
        .arg(
            Arg::with_name("asdl")
                .short("i")
                .long("input")
                .value_name("ASDL FILE")
                .help("Asdl file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("template")
                .short("t")
                .long("template")
                .value_name("TERA FILE")
                .help("Tera template file")
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("out")
                .value_name("OUT FILE")
                .help("Output file"),
        )
        .get_matches();
    let template_files = matches.values_of("template").unwrap().map(|t| Path::new(t)).collect();
    let asdl_file = matches.value_of("asdl").unwrap();
    let asdl = fs::read_to_string(asdl_file).unwrap();
    let output_file = matches.value_of("output").unwrap();
    let res = generate(&asdl, &template_files)?;
    fs::write(Path::new(output_file), res)?;
    Ok(())
}
