use crate::style::{Stylesheet, PYGMENTS_STYLESHEET};
use crate::{error, info};
use askama::Template;
use chrono::prelude::*;
use clap::ArgMatches;
use std::fs::remove_file;
use std::fs::File;
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
    pub style: String,
    pub css: &'static str,
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
    pub fn new(style: Stylesheet, language: &Languages) -> Header {
        Header {
            style: style.local(language),
            css: PYGMENTS_STYLESHEET,
        }
    }
}

impl Footer {
    pub fn new(name: String, matches: &ArgMatches) -> Footer {
        let local = Local::now();
        let language = Languages::parse(matches);
        let (date, text) = match language {
            Languages::EN => (
                local.format("%a, %b %e %Y").to_string(),
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

    pub fn to_file(&self) -> Result<(), IOError> {
        // check path
        let path = Path::new(TMP_PATH);

        if path.exists() {
            info("TMP file path is in use. Overriding");
            match remove_file(path) {
                Ok(_) => (),
                Err(e) => error(format!("Failed to remove old tmp file: {}", e)),
            };
        }

        // Create a file
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(e) => error(format!("Failed to create tmp file: {}", e)),
        };

        // Write footer data to the first handle.
        let text = match self.render() {
            Ok(text) => text,
            Err(e) => error(format!("Couldn't render footer: {}", e)),
        };
        match file.write_all(text.as_bytes()) {
            Ok(_) => (),
            Err(e) => error(format!("Failed to write to tmp file: {}", e)),
        };

        Ok(())
    }
}

impl Document {
    pub fn build(style: Stylesheet, content: String, matches: &ArgMatches) -> String {
        // create new document
        let new = Document {
            header: Header::new(style, &Languages::parse(&matches)),
            content,
        };

        // render document
        match new.render() {
            Ok(s) => s,
            Err(e) => error(format!("Couldn't render html: {}", e)),
        }
    }

    pub fn to_file(html: String) -> Result<(), IOError> {
        // check path
        let path = Path::new(TMP_DOCUMENT_PATH);

        if path.exists() {
            info("TMP file path is in use. Overriding");
            match remove_file(path) {
                Ok(_) => (),
                Err(e) => error(format!("Failed to remove old tmp file: {}", e)),
            };
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

        Ok(())
    }
}

// constant values
pub(crate) const TMP_PATH: &'static str = "./.footer.html";
pub(crate) const TMP_DOCUMENT_PATH: &'static str = "./.document.html";
