use crate::style::{Stylesheet, Themes};
use crate::{error, info, warning};
use askama::Template;
use chrono::prelude::*;
use clap::ArgMatches;
use std::fs::{read_dir, remove_file, File};
use std::io::{Error as IOError, Write};
use std::path::Path;

// trait for options
pub trait ClapOption {
    fn parse(matches: &ArgMatches) -> Self;
    fn options() -> &'static [&'static str];
}

// Languages to support different locales
pub enum Languages {
    EN,
    DE,
}

// Licenses to support direct attribution
pub enum CC4Licenses {
    BY,
    ByNc,
    BySa,
    ByNcSa,
    NONE,
}

/*  implementations for options */
impl ClapOption for Languages {
    fn parse(matches: &ArgMatches) -> Self {
        if matches.is_present("de") {
            Self::DE
        } else {
            Self::EN
        }
    }

    fn options() -> &'static [&'static str] {
        &["de", "en"]
    }
}

impl ClapOption for CC4Licenses {
    fn parse(matches: &ArgMatches) -> Self {
        match matches.value_of("license") {
            Some(l) => match l {
                "CC-BY-NC" => Self::ByNc,
                "CC-BY-SA" => Self::BySa,
                "CC-BY" => Self::BY,
                "CC-BY-NC-SA" => Self::ByNcSa,
                _ => Self::NONE,
            },
            None => Self::NONE,
        }
    }

    fn options() -> &'static [&'static str] {
        &["CC-BY-NC", "CC-BY", "CC-BY-NC-SA", "CC-BY-SA"]
    }
}

impl CC4Licenses {
    fn display(&self, language: &Languages) -> String {
        match self {
            Self::NONE => "".to_owned(),
            license => {
                let short = match license {
                    CC4Licenses::BY => "CC-BY",
                    CC4Licenses::BySa => "CC-BY-SA",
                    CC4Licenses::ByNcSa => "CC-BY-NC-SA",
                    CC4Licenses::ByNc => "CC-BY-NC",
                    CC4Licenses::NONE => unimplemented!(),
                };

                match *language {
                    Languages::DE => format!("- Lizenziert unter {} 4.0", short),
                    Languages::EN => format!("- Licensed under {} 4.0", short),
                }
            }
        }
    }
}

#[derive(Template)]
#[template(path = "header.html")]
pub struct Header {
    pub css: &'static str,
    pub theme: String,
    pub local: String,
}

#[derive(Template)]
#[template(path = "footer.html")]
pub struct Footer {
    pub date: String,
    pub name: String,
    pub text: String,
    pub license: String,
}

#[derive(Template)]
#[template(path = "document.html")]
pub struct Document {
    pub header: Header,
    pub content: String,
}

impl Header {
    pub fn new(style: Stylesheet, theme: String, language: &Languages) -> Header {
        Header {
            css: style.main,
            local: style.local(language),
            theme,
        }
    }
}

impl Footer {
    pub fn new(name: String, matches: &ArgMatches) -> Footer {
        let local = Local::now();
        let language = Languages::parse(matches);
        let (date, text) = match language {
            Languages::EN => (
                local.format("%b %e, %Y").to_string(),
                "Created by".to_owned(),
            ),
            Languages::DE => (
                local
                    .format_localized("%a, %e %b %Y", Locale::de_DE)
                    .to_string(),
                "Erstellt von".to_owned(),
            ),
        };

        Footer {
            name,
            date,
            text,
            license: CC4Licenses::parse(&matches).display(&language),
        }
    }

    pub fn to_file(&self) -> Result<String, IOError> {
        // check path
        let mut raw_path = FOOTER_PATH.to_owned();
        let mut path = Path::new(&raw_path);

        // when path exists already fall back to {number} - path
        if path.exists() {
            let mut i = 1;

            while path.exists() {
                raw_path = format!("./.footer-{}.html", i);
                path = Path::new(&raw_path);
                i += 1;
            }
        }

        // Create a file
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(e) => error(format!("Failed to create tmp file: {}", e)),
        };

        // Render Footer template into String
        let text = match self.render() {
            Ok(text) => text,
            Err(e) => error(format!("Couldn't render footer: {}", e)),
        };

        // Write Footer to tmp file
        match file.write_all(text.as_bytes()) {
            Ok(_) => (),
            Err(e) => error(format!("Failed to write to tmp file: {}", e)),
        };

        Ok(raw_path)
    }
}

impl Document {
    pub fn build(style: Stylesheet, content: String, matches: &ArgMatches) -> String {
        // create new document
        let new = Document {
            header: Header::new(style, Themes::parse(&matches), &Languages::parse(&matches)),
            content,
        };

        // render document
        match new.render() {
            Ok(s) => s,
            Err(e) => error(format!("Couldn't render document html: {}", e)),
        }
    }

    pub fn to_file(html: String) -> Result<String, IOError> {
        // check path
        let mut raw_path = DOCUMENT_PATH.to_owned();
        let mut path = Path::new(&raw_path);

        // when path exists already fall back to {number} - path
        if path.exists() {
            let mut i = 1;

            while path.exists() {
                raw_path = format!("./.document-{}.html", i);
                path = Path::new(&raw_path);
                i += 1;
            }
        }

        // Create a file
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(e) => error(format!("Failed to create tmp file: {}", e)),
        };

        // write document to file
        match file.write_all(html.as_bytes()) {
            Ok(_) => (),
            Err(e) => error(format!("Failed to write to tmp file: {}", e)),
        };

        Ok(raw_path)
    }

    // Removes footer and document artifacts
    pub fn remove_artifacts() -> Result<(), IOError> {
        for entry in read_dir(".")? {
            let entry = entry?;
            let path = entry.path();

            match path.to_str() {
                // only check if able to convert to string because there would be no way to determine without file name
                Some(stringified) => {
                    // check if is file and matching tmp file patterns
                    if path.is_file() && stringified.starts_with("./.document")
                        || stringified.starts_with("./.footer")
                    {
                        match remove_file(&path) {
                            Ok(_) => {
                                // print info if removing more than default tmp footer file
                                if stringified != FOOTER_PATH {
                                    info(format!("Removed old document artifact: {}", stringified));
                                }
                            }
                            Err(e) => {
                                // return in case of an error to caller
                                return Err(e);
                            }
                        }
                    }
                }
                // might add an option to surpress any warning later on
                None => warning("Failed to parse a part of the current dir"),
            };
        }

        Ok(())
    }
}

// static values
static DOCUMENT_PATH: &'static str = "./.document.html";
static FOOTER_PATH: &'static str = "./.footer.html";
