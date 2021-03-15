// includes
mod app;
mod convert;
mod document;
mod highlight;
mod style;

// imports
use ammonia::clean_text;
use ansi_term::Colour::{Blue, Red, Yellow};
use std::env::var;
use std::fmt::Display;
use std::fs::File;
use std::io::{stdout, Read};
use std::path::Path;
use std::process::exit;

fn error<S: Display>(e: S) -> ! {
    println!("{}: {}", Red.paint("[Error]"), e);
    exit(1);
}

fn callback_error<S: Display, F: Fn() -> ()>(e: S, callback: F) -> ! {
    callback();
    println!("\n{}: {}", Red.paint("[Error]"), e);
    exit(1)
}

fn info<S: Display>(message: S) {
    println!("{}: {}", Blue.paint("[Info]"), message);
}

fn warning<S: Display>(message: S) {
    println!("{}: {}", Yellow.paint("[Warning]"), message);
}

fn main() {
    let cli_app = app::app();
    let matches = cli_app.clone().get_matches();

    // check if subcommand
    if let Some(_) = matches.subcommand_matches("changelog") {
        // ATM the changelog is just embedded at build time and the printed to the user
        println!("{}", include_str!("../CHANGELOG.md"));
        exit(0)
    } else {
        // if input and output don't have default arguments the 'changelog'
        // subcommand would require input and output too
        if matches.value_of("INPUT").unwrap() == "EMPTY"
            || matches.value_of("OUTPUT").unwrap() == "EMPTY"
        {
            callback_error("Missing INPUT or OUTPUT argument", || {
                let mut out = stdout();
                cli_app.write_help(&mut out).unwrap();
            });
        }
    }

    // evaluate cli args
    let name = if matches.is_present("name") {
        Some(clean_text(&matches.value_of("name").unwrap().to_owned()))
    } else if var("NAME").is_ok() {
        Some(clean_text(&var("NAME").unwrap()))
    } else {
        None
    };

    let style = if matches.is_present("stylesheet") {
        let raw_path = matches.value_of("stylesheet").unwrap();
        let path = Path::new(raw_path);

        if !path.exists() {
            warning("Selected stylesheet wasn't found. Falling back to default");
            style::Stylesheet::default()
        } else if !path.is_file() {
            warning("Selected stylesheet isn't a file. Falling back to default");
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
    let output = highlight::parse_html(raw_input, options, matches.is_present("safe"));
    let rendered = document::Document::build(style, output, &matches);

    // convert html
    // this handles all errors with ! and doesn't return a result
    convert::convert(rendered, name, &matches)
}
