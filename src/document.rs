use crate::style::{Stylesheet, MDB_STYLESHEET};
use crate::{error, info};
use askama::Template;
use chrono::prelude::*;
use std::fs::remove_file;
use std::fs::File;
use std::io::{Error as IOError, Write};
use std::path::Path;

// Languages to support different locales
pub enum Languages {
    EN,
    DE,
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
            css: MDB_STYLESHEET,
            style: style.local(language),
        }
    }
}

impl Footer {
    pub fn new(name: String, language: &Languages) -> Footer {
        let local = Local::now();
        let (date, text) = match *language {
            Languages::EN => (
                local.format("%a %b %e %Y").to_string(),
                "Created by".to_owned(),
            ),
            Languages::DE => (
                local
                    .format_localized("%a %b %e %Y", Locale::de_DE)
                    .to_string(),
                "Erstellt von".to_owned(),
            ),
        };

        Footer { name, date, text }
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
    pub fn build(style: Stylesheet, content: String, language: &Languages) -> String {
        // create new document
        let new = Document {
            header: Header::new(style, language),
            content,
        };

        // render document
        match new.render() {
            Ok(s) => s,
            Err(e) => error(format!("Couldn't render html: {}", e)),
        }
    }
}

// constant values
pub(crate) const TMP_PATH: &'static str = "./.footer.html";
