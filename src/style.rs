use std::fs::File;
use crate::document::Languages;
use std::include_str;
use std::io::{BufReader, Error as IOError, Read};
use std::path::Path;

/* mdb base for basic layout */
pub const MDB_STYLESHEET: &'static str = include_str!("assets/css/mdb.min.css");

/* Stylesheets */
pub struct Stylesheet {
    pub en: String,
    pub de: String,
}

impl Stylesheet {
    pub fn default() -> Self {
        Stylesheet {
            en: include_str!("assets/css/style-en.css").to_owned(),
            de: include_str!("assets/css/style-de.css").to_owned(),
        }
    }

    pub fn local(&self, language: &Languages) -> String {
        match *language {
            Languages::DE => self.de.clone(),
            Languages::EN => self.en.clone()
        }
    }

    pub fn load(path: &Path) -> Result<Stylesheet, IOError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        // read into buffer
        reader.read_to_string(&mut buffer)?;

        Ok(Stylesheet {
            en: buffer.clone(),
            de: buffer,
        })
    }
}
