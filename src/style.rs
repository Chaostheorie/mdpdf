use crate::document::Languages;
use clap::{Arg, ArgMatches};
use std::fs::File;
use std::include_str;
use std::io::{BufReader, Error as IOError, Read};
use std::path::Path;

/* default name - can be included by having a name.txt file in src at compilation time */
static NAME: &'static str = include_str!("name.txt");

pub fn name_arg() -> Arg<'static, 'static> {
    let arg = Arg::with_name("name")
        .short("-n")
        .long("--name")
        .takes_value(true)
        .help("Add name and date to pdf footer");

    if NAME != "" {
        arg.default_value(NAME)
    } else {
        arg
    }
}

/* Stylesheets */
pub struct Stylesheet {
    pub en: String,
    pub de: String,
    pub main: &'static str,
}

pub struct Themes;

impl Themes {
    pub fn parse(matches: &ArgMatches) -> String {
        match matches.value_of("theme").unwrap() {
            "lime" => include_str!("assets/css/lime.css"),
            "night" => include_str!("assets/css/night.css"),
            _ => include_str!("assets/css/light.css"),
        }
        .to_owned()
    }
}

impl Stylesheet {
    pub fn default() -> Self {
        Stylesheet {
            en: include_str!("assets/css/en.css").to_owned(),
            de: include_str!("assets/css/de.css").to_owned(),
            main: MAIN_STYLESHEET,
        }
    }

    pub fn local(&self, language: &Languages) -> String {
        match *language {
            Languages::DE => self.de.clone(),
            Languages::EN => self.en.clone(),
        }
    }

    pub fn load(path: &Path) -> Result<Stylesheet, IOError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        // read into buffer
        reader.read_to_string(&mut buffer)?;

        Ok(Stylesheet {
            main: MAIN_STYLESHEET,
            en: buffer.clone(),
            de: buffer,
        })
    }
}

pub static MAIN_STYLESHEET: &'static str = include_str!("assets/css/main.css");
