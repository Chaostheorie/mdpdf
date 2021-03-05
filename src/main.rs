// includes
mod convert;
mod document;
mod style;

// imports
use ansi_term::Colour::{Blue, Yellow, Red};
use clap::{App, Arg};
use comrak::markdown_to_html;
use std::env::var;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::document::ClapOption;
use std::process::exit;

pub fn error<S: Display>(e: S) -> ! {
    println!("{}: {}", Red.paint("[Error]"), e);
    exit(1);
}

fn info<S: Display>(message: S) {
    println!("{}: {}", Blue.paint("[Info]"), message);
}

fn warning<S: Display>(message: S) {
    println!("{}: {}", Yellow.paint("[Warning]"), message);
}


fn main() {
    let matches = App::new("mdpdf")
        .version("0.1.1")
        .author("Cobalt <https://cobalt.rocks>")
        .about("Converts md to pdf with wkhtlmtopdf and comrak")
        .arg(
            Arg::with_name("INPUT")
                .takes_value(true)
                .required(true)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .takes_value(true)
                .required(true)
                .help("Sets the output path to write pdf to"),
        )
        .arg(   
            style::name_arg()
        )
        .arg(
            Arg::with_name("extensions")
            .help("Comrak Extensions to be used. By default all are activated. Commas are supported as separators when specifying multiple.")
            .takes_value(true)
            .possible_values(&["autolink", "description_lists", "footnotes", "superscript", "header_ids", "table", "tagfilter"  ,"tasklist"])
            .long("--extensions")
        )
        .arg(
            Arg::with_name("pagesize")
            .long("--pagesize")
            .takes_value(true)
            .help("PDF pagesize")
            .default_value("A4")
            .possible_values(&["A3", "A4", "A5", "A6"])
        )
        .arg(
            Arg::with_name("keep")
            .short("-k")
            .takes_value(false)
            .help("Keep tmp file")
        )
        .arg(
            Arg::with_name("orientation")
            .long("--orientation")        
            .help("PDF document orientation")
            .possible_values(&["landscape", "portrait"])
            .default_value("portrait")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("stylesheet")
            .short("-s")
            .long("--stylesheet")
            .takes_value(true)
            .help("Custom css stylesheet in addition to mdb")
        )
        .arg(
            Arg::with_name("de")
            .short("-d")
            .long("--german")
            .help("Static content in german")
        )
        .arg(
            Arg::with_name("toc")
            .help("Add table of contents (not implemented ATM")
            .long("--toc")
        )
        .arg(
            Arg::with_name("title")
            .long("--title")
            .short("-t")
            .takes_value(true)
            .help("PDF document title")
        )
        .arg(
            Arg::with_name("license")
            .short("-l")
            .long("--license")
            .help("Add CC 4.0 license to footer")
            .requires("name")
            .takes_value(true)
            .possible_values(document::CC4Licenses::options())
        )
        .get_matches();

    // evaluate cli args
    let name = if matches.is_present("name") {
        Some(matches.value_of("name").unwrap().to_owned())
    } else if var("NAME").is_ok() {
        Some(var("NAME").unwrap())
    } else {
        None
    };

    let style = if matches.is_present("stylesheet") {
        let raw_path = matches.value_of("stylesheet").unwrap();
        let path = Path::new(raw_path);

        if !path.exists() {
            info("Selected stylesheet wasn't found. Falling back to default");
            style::Stylesheet::default()
        } else if !path.is_file() {
            info("Selected stylesheet isn't a file. Falling back to default");
            style::Stylesheet::default()
        } else {
            match style::Stylesheet::load(&path) {
                Ok(stylesheet) => stylesheet,
                Err(e) => error(format!(
                    "Failed to embedded stylesheet. Possible corrupted binary: {}",
                    e
                )),
            }
        }
    } else {
        style::Stylesheet::default()
    };

    // check and evaluate input file
    let input_value = matches.value_of("INPUT").unwrap();
    let input_path = Path::new(input_value);

    // check if INPUT exists
    let mut input_file = if !input_path.exists() {
        error(format!("{} doesn't exist", input_value))
    } else if !input_path.is_file() {
        error(format!("{} must be a file", input_value))
    } else {
        match File::open(input_path) {
            Ok(file) => file,
            Err(e) => error(format!("Couldn't open source file: {}", e)),
        }
    };

    // read input file
    let mut raw_input = String::new();
    match input_file.read_to_string(&mut raw_input) {
        Ok(_) => (),
        Err(e) => error(format!("Couldn't load source file: {}", e)),
    };

    // create html
    let options = convert::build_options(&matches);
    let output = markdown_to_html(raw_input.as_str(), &options);
    let rendered = document::Document::build(style, output, &matches);

    // convert html
    // this handles all errors with ! and doesn't return a result
    convert::convert(rendered, name, &matches);
}
