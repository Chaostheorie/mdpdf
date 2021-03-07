use crate::document::{Document, Footer};
use crate::{error, info, warning};
use clap::ArgMatches;
use pulldown_cmark::Options;
use wkhtmltopdf::{Orientation, PageSize, PdfApplication, Size};

fn parse(wrapped: Option<&str>) -> PageSize {
    match wrapped {
        Some(size) => match size {
            "A3" => PageSize::A3,
            "A4" => PageSize::A4,
            "A5" => PageSize::A5,
            "A6" => PageSize::A6,
            _ => PageSize::A4,
        },
        None => PageSize::A4,
    }
}

pub fn convert(html: String, name: Option<String>, matches: &ArgMatches) {
    // create pdf application
    // this may initialize wkhtml too
    let mut app = match PdfApplication::new() {
        Ok(app) => app,
        Err(e) => error(format!("Failed to init PDF Application: {}", e)),
    };

    // evaluate title for the PDF output
    let title = match matches.value_of("title") {
        Some(title) => title,
        None => matches
            .value_of("INPUT")
            .unwrap()
            .strip_suffix(".md")
            .unwrap(),
    };

    // margin is not really important but may be useful when you intend to e.g. print a PDF later
    // on and want to put it into a folder. Useful for handouts too
    let margin = match matches.value_of("margin") {
        Some(margin) => match margin.parse::<u32>() {
            Ok(margin) => Size::Millimeters(margin),
            Err(_) => {
                info("Invalid Margin supplied");
                Size::Millimeters(10)
            }
        },
        None => Size::Millimeters(10),
    };

    // Not too sure if I may extend this part with support for angles
    let orientation = match matches.value_of("orientation") {
        Some(orientation) => match orientation {
            "portrait" => Orientation::Portrait,
            "landscape" => Orientation::Landscape,
            _ => Orientation::Portrait,
        },
        None => Orientation::Portrait,
    };

    let mut builder = app.builder();

    // save has_name & footer_path here to make the available later
    let has_name = name.is_some();

    if has_name {
        let footer = Footer::new(name.unwrap(), matches);
        let footer_path = match footer.to_file() {
            Ok(path) => path,
            Err(e) => error(format!("Failed to render footer: {}", e)),
        };

        if matches.is_present("print") {
            unsafe { builder.object_setting("margin.left", "12cm") };
        }

        let out_result = unsafe {
            builder
                .orientation(orientation)
                .object_setting("footer.htmlUrl", footer_path) // pretty sure this isn't totally safe
                .object_setting("load.blockLocalFileAccess", "false")
                .object_setting("web.enableJavascript", "true")
                .margin(margin)
                .page_size(parse(matches.value_of("pagesize")))
                .title(&title)
                .build_from_html(&html)
        };

        let mut out = match out_result {
            Ok(pdf) => pdf,
            Err(e) => error(format!("Failed to generate PDF: {}", e)),
        };

        let path = matches.value_of("OUTPUT").unwrap();
        match out.save(path) {
            Ok(_) => info(format!("Generated PDF and saved to {}", path)),
            Err(e) => error(format!("Failed to save PDF to {}: {}", path, e)),
        };
    } else {
        if matches.is_present("print") {
            unsafe { builder.object_setting("margin.left", "12cm") };
        }

        let out_result = unsafe {
            builder
                .orientation(orientation)
                .margin(margin)
                .page_size(parse(matches.value_of("pagesize")))
                .object_setting("load.blockLocalFileAccess", "false")
                .object_setting("web.enableJavascript", "true")
                .title(&title)
                .build_from_html(&html)
        };

        let mut out = match out_result {
            Ok(pdf) => pdf,
            Err(e) => error(format!("Failed to generate PDF: {}", e)),
        };

        let path = matches.value_of("OUTPUT").unwrap();
        match out.save(path) {
            Ok(_) => info(format!("Generated PDF and saved to {}", path)),
            Err(e) => error(format!("Failed to save PDF to {}: {}", path, e)),
        };
    }

    if !matches.is_present("keep") {
        match Document::remove_artifacts() {
            Ok(_) => (),
            Err(e) => warning(format!("Failed to remove old document artifact: {}", e)),
        };
    } else {
        match Document::to_file(html) {
            Ok(path) => info(format!("Kept document body under: {}", path)),
            Err(e) => error(format!(
                "Failed to render tmp document (try without -k): {}",
                e
            )),
        };
    }
}

pub fn build_options(matches: &ArgMatches) -> Options {
    let mut options = Options::empty();

    if matches.is_present("extensions") {
        // extract data and prepare new options obj
        let activated_extensions: Vec<&str> =
            matches.value_of("extensions").unwrap().split(",").collect();
        // check for extensions

        // check for strikethrough extension
        options.set(
            Options::ENABLE_STRIKETHROUGH,
            activated_extensions.contains(&"strikethrough"),
        );

        // check for table extension
        options.set(
            Options::ENABLE_TABLES,
            activated_extensions.contains(&"table"),
        );

        // check for taskslists extension
        options.set(
            Options::ENABLE_TASKLISTS,
            activated_extensions.contains(&"tasklist"),
        );

        // check for footnote extension
        options.set(
            Options::ENABLE_FOOTNOTES,
            activated_extensions.contains(&"footnotes"),
        );

        // check for smart punctuation
        options.set(
            Options::ENABLE_SMART_PUNCTUATION,
            activated_extensions.contains(&"smart-punctuation"),
        );
    } else {
        // enable all extensions
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
    };

    options
}
